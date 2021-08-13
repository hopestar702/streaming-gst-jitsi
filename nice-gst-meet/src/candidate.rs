// Generated by gir (https://github.com/gtk-rs/gir @ 5bbf6cb)
// from ../../gir-files (@ 8e47c67)
// DO NOT EDIT

use std::{ffi::CStr, net::SocketAddr};

use glib::translate::*;
use libc::c_char;
use nice_sys as ffi;
use nix::sys::socket::{AddressFamily, InetAddr};

#[cfg(any(feature = "v0_1_18", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v0_1_18")))]
use crate::CandidateTransport;
use crate::CandidateType;

glib::wrapper! {
    #[derive(PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct Candidate(Boxed<ffi::NiceCandidate>);

    match fn {
        copy => |ptr| ffi::nice_candidate_copy(ptr),
        free => |ptr| ffi::nice_candidate_free(ptr),
        type_ => || ffi::nice_candidate_get_type(),
    }
}

impl ::std::fmt::Debug for Candidate {
  fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
    f.debug_struct("Candidate")
      .field("type_", &self.type_())
      .field("foundation", &self.foundation())
      .field("transport", &self.transport())
      .field("addr", &self.addr())
      .field("priority", &self.priority())
      .field("stream_id", &self.stream_id())
      .field("component_id", &self.component_id())
      .field("username", &self.username())
      .field("password", &self.password())
      .finish()
  }
}

unsafe impl Send for Candidate {}

impl<'a> ToGlibPtr<'a, *mut ffi::NiceCandidate> for Candidate {
  type Storage = &'a Self;

  #[inline]
  fn to_glib_none(&'a self) -> Stash<'a, *mut ffi::NiceCandidate, Self> {
    Stash(&*self.0 as *const _ as *mut _, self)
  }
}

impl Candidate {
  #[doc(alias = "nice_candidate_new")]
  pub fn new(type_: CandidateType) -> Candidate {
    unsafe { from_glib_full(ffi::nice_candidate_new(type_.into_glib())) }
  }

  pub fn type_(&self) -> CandidateType {
    unsafe { CandidateType::from_glib(self.0.type_) }
  }

  #[cfg(any(feature = "v0_1_18", feature = "dox"))]
  #[cfg_attr(feature = "dox", doc(cfg(feature = "v0_1_18")))]
  pub fn transport(&self) -> CandidateTransport {
    unsafe { CandidateTransport::from_glib(self.0.transport) }
  }

  #[cfg(any(feature = "v0_1_18", feature = "dox"))]
  #[cfg_attr(feature = "dox", doc(cfg(feature = "v0_1_18")))]
  pub fn set_transport(&mut self, transport: CandidateTransport) {
    self.0.transport = transport.into_glib();
  }

  pub fn addr(&self) -> SocketAddr {
    unsafe {
      match AddressFamily::from_i32(self.0.addr.s.addr.sa_family as i32).unwrap() {
        AddressFamily::Inet => InetAddr::V4(self.0.addr.s.ip4).to_std(),
        AddressFamily::Inet6 => InetAddr::V6(self.0.addr.s.ip6).to_std(),
        other => panic!("unsupported address family: {:?}", other),
      }
    }
  }

  pub fn set_addr(&mut self, addr: SocketAddr) {
    match InetAddr::from_std(&addr) {
      InetAddr::V4(ip4) => unsafe {
        ffi::nice_address_set_ipv4(
          &mut self.0.addr as *mut _,
          u32::from_be(ip4.sin_addr.s_addr),
        );
        ffi::nice_address_set_port(
          &mut self.0.addr as *mut _,
          u16::from_be(ip4.sin_port) as u32,
        );
      },
      InetAddr::V6(ip6) => unsafe {
        ffi::nice_address_set_ipv6(
          &mut self.0.addr as *mut _,
          &ip6.sin6_addr.s6_addr as *const _,
        );
        ffi::nice_address_set_port(
          &mut self.0.addr as *mut _,
          u16::from_be(ip6.sin6_port) as u32,
        );
      },
    }
  }

  pub fn priority(&self) -> u32 {
    self.0.priority
  }

  pub fn set_priority(&mut self, priority: u32) {
    self.0.priority = priority;
  }

  pub fn stream_id(&self) -> u32 {
    self.0.stream_id
  }

  pub fn set_stream_id(&mut self, stream_id: u32) {
    self.0.stream_id = stream_id;
  }

  pub fn component_id(&self) -> u32 {
    self.0.component_id
  }

  pub fn set_component_id(&mut self, component_id: u32) {
    self.0.component_id = component_id;
  }

  pub fn foundation(&self) -> Result<&str, std::str::Utf8Error> {
    unsafe { CStr::from_ptr(&self.0.foundation as *const c_char).to_str() }
  }

  pub fn set_foundation(&mut self, foundation: &str) {
    let mut bytes: Vec<_> = foundation
      .as_bytes()
      .iter()
      .take(32)
      .map(|c| *c as i8)
      .collect();
    bytes.resize(33, 0);
    self.0.foundation.copy_from_slice(&bytes);
  }

  pub fn username(&self) -> Result<&str, std::str::Utf8Error> {
    if self.0.username.is_null() {
      Ok("")
    }
    else {
      unsafe { CStr::from_ptr(self.0.username).to_str() }
    }
  }

  pub fn set_username(&mut self, username: &str) {
    self.0.username = username.to_owned().to_glib_full();
  }

  pub fn password(&self) -> Result<&str, std::str::Utf8Error> {
    if self.0.password.is_null() {
      Ok("")
    }
    else {
      unsafe { CStr::from_ptr(self.0.password).to_str() }
    }
  }

  pub fn set_password(&mut self, password: &str) {
    self.0.password = password.to_owned().to_glib_full();
  }

  #[cfg(any(feature = "v0_1_15", feature = "dox"))]
  #[cfg_attr(feature = "dox", doc(cfg(feature = "v0_1_15")))]
  #[doc(alias = "nice_candidate_equal_target")]
  pub fn equal_target(&self, candidate2: &Candidate) -> bool {
    unsafe {
      from_glib(ffi::nice_candidate_equal_target(
        self.to_glib_none().0,
        candidate2.to_glib_none().0,
      ))
    }
  }

  #[cfg(any(feature = "v0_1_18", feature = "dox"))]
  #[cfg_attr(feature = "dox", doc(cfg(feature = "v0_1_18")))]
  #[doc(alias = "nice_candidate_transport_to_string")]
  pub fn transport_to_string(transport: CandidateTransport) -> Option<String> {
    unsafe {
      from_glib_none(ffi::nice_candidate_transport_to_string(
        transport.into_glib(),
      ))
    }
  }

  #[cfg(any(feature = "v0_1_18", feature = "dox"))]
  #[cfg_attr(feature = "dox", doc(cfg(feature = "v0_1_18")))]
  #[doc(alias = "nice_candidate_type_to_string")]
  pub fn type_to_string(type_: CandidateType) -> Option<String> {
    unsafe { from_glib_none(ffi::nice_candidate_type_to_string(type_.into_glib())) }
  }
}
