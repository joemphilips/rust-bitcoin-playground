pub struct Builder {}


/// command for wallet to specify to builder.
pub enum BuilderCommand {
  GatherCoins,
  CreateRawTx,
  AddOutputToTx,
  SignTx,
  SendTx,
  BroadCastTx,
}


trait Command<T> {
  fn execute(&self);
}


impl Command<Builder> for BuilderCommand {
  fn execute(&self) {
    use self::BuilderCommand::*;
    match *self {
      // TODO: implement other Commands
      SignTx => println!("not implemented yet!"),
      _ => unreachable!(),
    }
  }
}
