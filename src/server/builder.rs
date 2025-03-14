use std::net::IpAddr;

use super::Server;

pub struct Builder {
  server: Server,
}

impl Builder {
  pub fn new() -> Self {
    Builder {
      server: Default::default(),
    }
  }

  pub fn build(self) -> Server {
    self.server
  }

  pub fn host<Host>(&mut self, host: Host) -> &mut Self
  where
    Host: Into<IpAddr>,
  {
    self.server.host = host.into();
    self
  }

  pub fn port<Port>(&mut self, port: Port) -> &mut Self
  where
    Port: Into<u16>,
  {
    self.server.port = port.into();
    self
  }

  pub fn api_mount<Mount>(&mut self, mount: Mount) -> &mut Self
  where
    Mount: Into<String>,
  {
    self.server.api_mount = mount.into();
    self
  }

  pub fn swagger_ui_mount<Mount>(&mut self, mount: Mount) -> &mut Self
  where
    Mount: Into<String>,
  {
    self.server.swagger_ui_mount = mount.into();
    self
  }

  pub fn api_docs_mount<Mount>(&mut self, mount: Mount) -> &mut Self
  where
    Mount: Into<String>,
  {
    self.server.api_docs_mount = mount.into();
    self
  }

  pub fn swagger_ui<Flag>(&mut self, flag: Flag) -> &mut Self
  where
    Flag: Into<bool>,
  {
    self.server.swagger_ui = flag.into();
    self
  }
}
