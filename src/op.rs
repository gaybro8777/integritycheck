#[derive(Debug)]
pub enum Operation {
  Acknowledge,
  Diff,
  Verify,
  History,
  Initialize
}

pub trait OperationHelp {
  fn usage(self: &Self) -> String;
}

impl Operation {

  pub fn from_str(str: &str) -> Option<Operation> {
    return match str {
      "ack" => Some(Operation::Acknowledge),
      "diff" => Some(Operation::Diff),
      "verify" => Some(Operation::Verify),
      "init" => Some(Operation::Initialize),
      "log" => Some(Operation::History),
      _ => None,
    };
  }

}
