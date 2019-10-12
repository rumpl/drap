use crate::cnab::Bundle;
use clap::{App, Arg, ArgMatches};
use shiplift::rep::ContainerCreateInfo;
use shiplift::{tty::StreamType, ContainerOptions, Docker, Error, ExecContainerOptions};
use tokio::prelude::{Future, Stream};

use std::path::Path;
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

fn toto(
  docker: &Docker,
  container_options: &ContainerOptions,
) -> impl Future<Item = ContainerCreateInfo, Error = Error> {
  docker.containers().create(&container_options)
}

fn run(args: &ArgMatches<'_>) {
  let file = args.value_of("INPUT").unwrap();
  let bundle = Bundle::from_file(Path::new(file)).unwrap();
  let docker = Docker::new();
  // let options = ExecContainerOptions::builder()
  //   .cmd(vec!["/cnab/app/run", "install"])
  //   .attach_stderr(true)
  //   .attach_stderr(true)
  //   .build();

  let container_options = ContainerOptions::builder(&bundle.invocation_images[0].image).build();

  let mut runtime = tokio::runtime::Runtime::new().expect("Unable to create a runtime");
  let info = runtime.block_on(toto(&docker, &container_options)).unwrap();

  let fut = docker
    .containers()
    .get(&info.id)
    .start()
    .map_err(|e| eprintln!("Error: {}", e));

  tokio::run(fut);
}
