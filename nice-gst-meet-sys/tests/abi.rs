// Generated by gir (https://github.com/gtk-rs/gir @ 5bbf6cb)
// from ../../gir-files (@ 8e47c67)
// DO NOT EDIT

use std::{
  env,
  error::Error,
  ffi::OsString,
  mem::{align_of, size_of},
  path::Path,
  process::Command,
  str,
};

use nice_sys::*;
use tempfile::Builder;

static PACKAGES: &[&str] = &["nice"];

#[derive(Clone, Debug)]
struct Compiler {
  pub args: Vec<String>,
}

impl Compiler {
  pub fn new() -> Result<Self, Box<dyn Error>> {
    let mut args = get_var("CC", "cc")?;
    args.push("-Wno-deprecated-declarations".to_owned());
    // For _Generic
    args.push("-std=c11".to_owned());
    // For %z support in printf when using MinGW.
    args.push("-D__USE_MINGW_ANSI_STDIO".to_owned());
    args.extend(get_var("CFLAGS", "")?);
    args.extend(get_var("CPPFLAGS", "")?);
    args.extend(pkg_config_cflags(PACKAGES)?);
    Ok(Self { args })
  }

  pub fn compile(&self, src: &Path, out: &Path) -> Result<(), Box<dyn Error>> {
    let mut cmd = self.to_command();
    cmd.arg(src);
    cmd.arg("-o");
    cmd.arg(out);
    let status = cmd.spawn()?.wait()?;
    if !status.success() {
      return Err(format!("compilation command {:?} failed, {}", &cmd, status).into());
    }
    Ok(())
  }

  fn to_command(&self) -> Command {
    let mut cmd = Command::new(&self.args[0]);
    cmd.args(&self.args[1..]);
    cmd
  }
}

fn get_var(name: &str, default: &str) -> Result<Vec<String>, Box<dyn Error>> {
  match env::var(name) {
    Ok(value) => Ok(shell_words::split(&value)?),
    Err(env::VarError::NotPresent) => Ok(shell_words::split(default)?),
    Err(err) => Err(format!("{} {}", name, err).into()),
  }
}

fn pkg_config_cflags(packages: &[&str]) -> Result<Vec<String>, Box<dyn Error>> {
  if packages.is_empty() {
    return Ok(Vec::new());
  }
  let pkg_config = env::var_os("PKG_CONFIG").unwrap_or_else(|| OsString::from("pkg-config"));
  let mut cmd = Command::new(pkg_config);
  cmd.arg("--cflags");
  cmd.args(packages);
  let out = cmd.output()?;
  if !out.status.success() {
    return Err(format!("command {:?} returned {}", &cmd, out.status).into());
  }
  let stdout = str::from_utf8(&out.stdout)?;
  Ok(shell_words::split(stdout.trim())?)
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct Layout {
  size: usize,
  alignment: usize,
}

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
struct Results {
  /// Number of successfully completed tests.
  passed: usize,
  /// Total number of failed tests (including those that failed to compile).
  failed: usize,
}

impl Results {
  fn record_passed(&mut self) {
    self.passed += 1;
  }

  fn record_failed(&mut self) {
    self.failed += 1;
  }

  fn summary(&self) -> String {
    format!("{} passed; {} failed", self.passed, self.failed)
  }

  fn expect_total_success(&self) {
    if self.failed == 0 {
      println!("OK: {}", self.summary());
    }
    else {
      panic!("FAILED: {}", self.summary());
    };
  }
}

#[test]
fn cross_validate_constants_with_c() {
  let mut c_constants: Vec<(String, String)> = Vec::new();

  for l in get_c_output("constant").unwrap().lines() {
    let mut words = l.trim().split(';');
    let name = words.next().expect("Failed to parse name").to_owned();
    let value = words
      .next()
      .and_then(|s| s.parse().ok())
      .expect("Failed to parse value");
    c_constants.push((name, value));
  }

  let mut results = Results::default();

  for ((rust_name, rust_value), (c_name, c_value)) in RUST_CONSTANTS.iter().zip(c_constants.iter())
  {
    if rust_name != c_name {
      results.record_failed();
      eprintln!("Name mismatch:\nRust: {:?}\nC:    {:?}", rust_name, c_name,);
      continue;
    }

    if rust_value != c_value {
      results.record_failed();
      eprintln!(
        "Constant value mismatch for {}\nRust: {:?}\nC:    {:?}",
        rust_name, rust_value, &c_value
      );
      continue;
    }

    results.record_passed();
  }

  results.expect_total_success();
}

#[test]
fn cross_validate_layout_with_c() {
  let mut c_layouts = Vec::new();

  for l in get_c_output("layout").unwrap().lines() {
    let mut words = l.trim().split(';');
    let name = words.next().expect("Failed to parse name").to_owned();
    let size = words
      .next()
      .and_then(|s| s.parse().ok())
      .expect("Failed to parse size");
    let alignment = words
      .next()
      .and_then(|s| s.parse().ok())
      .expect("Failed to parse alignment");
    c_layouts.push((name, Layout { size, alignment }));
  }

  let mut results = Results::default();

  for ((rust_name, rust_layout), (c_name, c_layout)) in RUST_LAYOUTS.iter().zip(c_layouts.iter()) {
    if rust_name != c_name {
      results.record_failed();
      eprintln!("Name mismatch:\nRust: {:?}\nC:    {:?}", rust_name, c_name,);
      continue;
    }

    if rust_layout != c_layout {
      results.record_failed();
      eprintln!(
        "Layout mismatch for {}\nRust: {:?}\nC:    {:?}",
        rust_name, rust_layout, &c_layout
      );
      continue;
    }

    results.record_passed();
  }

  results.expect_total_success();
}

fn get_c_output(name: &str) -> Result<String, Box<dyn Error>> {
  let tmpdir = Builder::new().prefix("abi").tempdir()?;
  let exe = tmpdir.path().join(name);
  let c_file = Path::new("tests").join(name).with_extension("c");

  let cc = Compiler::new().expect("configured compiler");
  cc.compile(&c_file, &exe)?;

  let mut abi_cmd = Command::new(exe);
  let output = abi_cmd.output()?;
  if !output.status.success() {
    return Err(format!("command {:?} failed, {:?}", &abi_cmd, &output).into());
  }

  Ok(String::from_utf8(output.stdout)?)
}

const RUST_LAYOUTS: &[(&str, Layout)] = &[
  (
    "NiceAddress",
    Layout {
      size: size_of::<NiceAddress>(),
      alignment: align_of::<NiceAddress>(),
    },
  ),
  (
    "NiceAgentClass",
    Layout {
      size: size_of::<NiceAgentClass>(),
      alignment: align_of::<NiceAgentClass>(),
    },
  ),
  (
    "NiceAgentOption",
    Layout {
      size: size_of::<NiceAgentOption>(),
      alignment: align_of::<NiceAgentOption>(),
    },
  ),
  (
    "NiceCandidate",
    Layout {
      size: size_of::<NiceCandidate>(),
      alignment: align_of::<NiceCandidate>(),
    },
  ),
  (
    "NiceCandidateTransport",
    Layout {
      size: size_of::<NiceCandidateTransport>(),
      alignment: align_of::<NiceCandidateTransport>(),
    },
  ),
  (
    "NiceCandidateType",
    Layout {
      size: size_of::<NiceCandidateType>(),
      alignment: align_of::<NiceCandidateType>(),
    },
  ),
  (
    "NiceCompatibility",
    Layout {
      size: size_of::<NiceCompatibility>(),
      alignment: align_of::<NiceCompatibility>(),
    },
  ),
  (
    "NiceComponentState",
    Layout {
      size: size_of::<NiceComponentState>(),
      alignment: align_of::<NiceComponentState>(),
    },
  ),
  (
    "NiceComponentType",
    Layout {
      size: size_of::<NiceComponentType>(),
      alignment: align_of::<NiceComponentType>(),
    },
  ),
  (
    "NiceInputMessage",
    Layout {
      size: size_of::<NiceInputMessage>(),
      alignment: align_of::<NiceInputMessage>(),
    },
  ),
  (
    "NiceNominationMode",
    Layout {
      size: size_of::<NiceNominationMode>(),
      alignment: align_of::<NiceNominationMode>(),
    },
  ),
  (
    "NiceOutputMessage",
    Layout {
      size: size_of::<NiceOutputMessage>(),
      alignment: align_of::<NiceOutputMessage>(),
    },
  ),
  (
    "NiceProxyType",
    Layout {
      size: size_of::<NiceProxyType>(),
      alignment: align_of::<NiceProxyType>(),
    },
  ),
  (
    "NiceRelayType",
    Layout {
      size: size_of::<NiceRelayType>(),
      alignment: align_of::<NiceRelayType>(),
    },
  ),
  (
    "PseudoTcpCallbacks",
    Layout {
      size: size_of::<PseudoTcpCallbacks>(),
      alignment: align_of::<PseudoTcpCallbacks>(),
    },
  ),
  (
    "PseudoTcpDebugLevel",
    Layout {
      size: size_of::<PseudoTcpDebugLevel>(),
      alignment: align_of::<PseudoTcpDebugLevel>(),
    },
  ),
  (
    "PseudoTcpShutdown",
    Layout {
      size: size_of::<PseudoTcpShutdown>(),
      alignment: align_of::<PseudoTcpShutdown>(),
    },
  ),
  (
    "PseudoTcpState",
    Layout {
      size: size_of::<PseudoTcpState>(),
      alignment: align_of::<PseudoTcpState>(),
    },
  ),
  (
    "PseudoTcpWriteResult",
    Layout {
      size: size_of::<PseudoTcpWriteResult>(),
      alignment: align_of::<PseudoTcpWriteResult>(),
    },
  ),
];

const RUST_CONSTANTS: &[(&str, &str)] = &[
  ("NICE_AGENT_MAX_REMOTE_CANDIDATES", "25"),
  ("(guint) NICE_AGENT_OPTION_CONSENT_FRESHNESS", "32"),
  ("(guint) NICE_AGENT_OPTION_ICE_TRICKLE", "8"),
  ("(guint) NICE_AGENT_OPTION_LITE_MODE", "4"),
  ("(guint) NICE_AGENT_OPTION_REGULAR_NOMINATION", "1"),
  ("(guint) NICE_AGENT_OPTION_RELIABLE", "2"),
  ("(guint) NICE_AGENT_OPTION_SUPPORT_RENOMINATION", "16"),
  ("NICE_CANDIDATE_MAX_FOUNDATION", "33"),
  ("NICE_CANDIDATE_MAX_LOCAL_ADDRESSES", "64"),
  ("NICE_CANDIDATE_MAX_TURN_SERVERS", "8"),
  ("(gint) NICE_CANDIDATE_TRANSPORT_TCP_ACTIVE", "1"),
  ("(gint) NICE_CANDIDATE_TRANSPORT_TCP_PASSIVE", "2"),
  ("(gint) NICE_CANDIDATE_TRANSPORT_TCP_SO", "3"),
  ("(gint) NICE_CANDIDATE_TRANSPORT_UDP", "0"),
  ("(gint) NICE_CANDIDATE_TYPE_HOST", "0"),
  ("(gint) NICE_CANDIDATE_TYPE_PEER_REFLEXIVE", "2"),
  ("(gint) NICE_CANDIDATE_TYPE_RELAYED", "3"),
  ("(gint) NICE_CANDIDATE_TYPE_SERVER_REFLEXIVE", "1"),
  ("(gint) NICE_COMPATIBILITY_DRAFT19", "0"),
  ("(gint) NICE_COMPATIBILITY_GOOGLE", "1"),
  ("(gint) NICE_COMPATIBILITY_LAST", "5"),
  ("(gint) NICE_COMPATIBILITY_MSN", "2"),
  ("(gint) NICE_COMPATIBILITY_OC2007", "4"),
  ("(gint) NICE_COMPATIBILITY_OC2007R2", "5"),
  ("(gint) NICE_COMPATIBILITY_RFC5245", "0"),
  ("(gint) NICE_COMPATIBILITY_WLM2009", "3"),
  ("(gint) NICE_COMPONENT_STATE_CONNECTED", "3"),
  ("(gint) NICE_COMPONENT_STATE_CONNECTING", "2"),
  ("(gint) NICE_COMPONENT_STATE_DISCONNECTED", "0"),
  ("(gint) NICE_COMPONENT_STATE_FAILED", "5"),
  ("(gint) NICE_COMPONENT_STATE_GATHERING", "1"),
  ("(gint) NICE_COMPONENT_STATE_LAST", "6"),
  ("(gint) NICE_COMPONENT_STATE_READY", "4"),
  ("(gint) NICE_COMPONENT_TYPE_RTCP", "2"),
  ("(gint) NICE_COMPONENT_TYPE_RTP", "1"),
  ("(gint) NICE_NOMINATION_MODE_AGGRESSIVE", "1"),
  ("(gint) NICE_NOMINATION_MODE_REGULAR", "0"),
  ("(gint) NICE_PROXY_TYPE_HTTP", "2"),
  ("(gint) NICE_PROXY_TYPE_LAST", "2"),
  ("(gint) NICE_PROXY_TYPE_NONE", "0"),
  ("(gint) NICE_PROXY_TYPE_SOCKS5", "1"),
  ("(gint) NICE_RELAY_TYPE_TURN_TCP", "1"),
  ("(gint) NICE_RELAY_TYPE_TURN_TLS", "2"),
  ("(gint) NICE_RELAY_TYPE_TURN_UDP", "0"),
  ("(gint) PSEUDO_TCP_CLOSED", "4"),
  ("(gint) PSEUDO_TCP_CLOSE_WAIT", "9"),
  ("(gint) PSEUDO_TCP_CLOSING", "7"),
  ("(gint) PSEUDO_TCP_DEBUG_NONE", "0"),
  ("(gint) PSEUDO_TCP_DEBUG_NORMAL", "1"),
  ("(gint) PSEUDO_TCP_DEBUG_VERBOSE", "2"),
  ("(gint) PSEUDO_TCP_ESTABLISHED", "3"),
  ("(gint) PSEUDO_TCP_FIN_WAIT_1", "5"),
  ("(gint) PSEUDO_TCP_FIN_WAIT_2", "6"),
  ("(gint) PSEUDO_TCP_LAST_ACK", "10"),
  ("(gint) PSEUDO_TCP_LISTEN", "0"),
  ("(gint) PSEUDO_TCP_SHUTDOWN_RD", "0"),
  ("(gint) PSEUDO_TCP_SHUTDOWN_RDWR", "2"),
  ("(gint) PSEUDO_TCP_SHUTDOWN_WR", "1"),
  ("(gint) PSEUDO_TCP_SYN_RECEIVED", "2"),
  ("(gint) PSEUDO_TCP_SYN_SENT", "1"),
  ("(gint) PSEUDO_TCP_TIME_WAIT", "8"),
  ("(gint) WR_FAIL", "2"),
  ("(gint) WR_SUCCESS", "0"),
  ("(gint) WR_TOO_LARGE", "1"),
];
