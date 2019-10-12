use crate::cnab::Bundle;
use clap::{App, Arg, ArgMatches};
use http::Uri;
use shiplift::rep::ContainerCreateInfo;
use shiplift::{ContainerOptions, Docker, Error};
use std::path::Path;
use tokio::prelude::{Future, Stream};
use tokio::runtime::current_thread::Runtime;

pub fn cli() -> App<'static, 'static> {
  App::new("run")
    .about("Run an application")
    .arg(Arg::with_name("INPUT").required(true))
}

pub fn exec(args: &ArgMatches<'_>) -> Option<fn()> {
  run(args);
  fn noop() {};
  Some(noop)
}

fn container_create(
  docker: &Docker,
  container_options: &ContainerOptions,
) -> impl Future<Item = ContainerCreateInfo, Error = Error> {
  docker.containers().create(container_options)
}

fn run(args: &ArgMatches<'_>) {
  let file = args.value_of("INPUT").unwrap();
  let bundle = Bundle::from_file(Path::new(file)).unwrap();
  // Needs to be on tcp because shiplift can't attach over the socket.
  let docker = Docker::host("tcp://127.0.0.1:1234".parse::<Uri>().unwrap());
  let container_options = ContainerOptions::builder(&bundle.invocation_images[0].image)
    .cmd(vec!["/bin/sh", "-c", "/cnab/app/run"])
    .env(vec![
      "CNAB_ACTION=install",
      "CNAB_INSTALLATION_NAME=app",
      "CNAB_BUNDLE_NAME=bundle",
      "CNAB_BUNDLE_VERSION=1.0.0",
    ])
    .attach_stderr(true)
    .attach_stdout(true)
    .build();

  let mut runtime = tokio::runtime::Runtime::new().expect("Unable to create a runtime");
  let info = runtime
    .block_on(container_create(&docker, &container_options))
    .unwrap();

  let mut rt = Runtime::new().unwrap();
  let cont = docker.containers().get(&info.id);

  let fut = cont.attach().and_then(|multiplexed| {
    multiplexed.for_each(|c| {
      println!("{}", c.as_string_lossy());
      Ok(())
    })
  });

  let f = cont.start();
  rt.block_on(f).unwrap();

  rt.block_on(fut).unwrap();
}
