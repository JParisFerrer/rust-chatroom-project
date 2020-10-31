// @generated, do not edit
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct JoinRequest {
  pub username: ::std::string::String,
}
impl ::std::default::Default for JoinRequest {
  fn default() -> Self {
    JoinRequest {
      username: ::std::default::Default::default(),
    }
  }
}
lazy_static! {
  pub static ref JoinRequest_default: JoinRequest = JoinRequest::default();
}
impl ::pb_jelly::Message for JoinRequest {
  fn compute_size(&self) -> usize  {
    let mut size = 0;
    let mut username_size = 0;
    if self.username != <::std::string::String as ::std::default::Default>::default() {
      let val = &self.username;
      let l = ::pb_jelly::Message::compute_size(val);
      username_size += ::pb_jelly::wire_format::serialized_length(1);
      username_size += ::pb_jelly::varint::serialized_length(l as u64);
      username_size += l;
    }
    size += username_size;
    size
  }
  fn compute_grpc_slices_size(&self) -> usize  {
    let mut size = 0;
    if self.username != <::std::string::String as ::std::default::Default>::default() {
      let val = &self.username;
      size += ::pb_jelly::Message::compute_grpc_slices_size(val);
    }
    size
  }
  fn serialize<W: ::pb_jelly::PbBufferWriter>(&self, w: &mut W) -> ::std::io::Result<()> {
    if self.username != <::std::string::String as ::std::default::Default>::default() {
      let val = &self.username;
      ::pb_jelly::wire_format::write(1, ::pb_jelly::wire_format::Type::LengthDelimited, w)?;
      let l = ::pb_jelly::Message::compute_size(val);
      ::pb_jelly::varint::write(l as u64, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    Ok(())
  }
  fn deserialize<B: ::pb_jelly::PbBufferReader>(&mut self, mut buf: &mut B) -> ::std::io::Result<()> {
    while let Some((field_number, typ)) = ::pb_jelly::wire_format::read(&mut buf)? {
      match field_number {
        1 => {
          ::pb_jelly::ensure_wire_format(typ, ::pb_jelly::wire_format::Type::LengthDelimited, "JoinRequest", 1)?;
          let len = ::pb_jelly::varint::ensure_read(&mut buf)?;
          let mut next = ::pb_jelly::ensure_split(buf, len as usize)?;
          let mut val: ::std::string::String = ::std::default::Default::default();
          ::pb_jelly::Message::deserialize(&mut val, &mut next)?;
          self.username = val;
        }
        _ => {
          ::pb_jelly::skip(typ, &mut buf)?;
        }
      }
    }
    Ok(())
  }
}
impl ::pb_jelly::MessageDescriptor for JoinRequest {
  const NAME: &'static str = "JoinRequest";
  const FULL_NAME: &'static str = "chatroom.JoinRequest";
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct JoinResponse {
  pub initial_chats: ::std::vec::Vec<::std::string::String>,
}
impl ::std::default::Default for JoinResponse {
  fn default() -> Self {
    JoinResponse {
      initial_chats: ::std::default::Default::default(),
    }
  }
}
lazy_static! {
  pub static ref JoinResponse_default: JoinResponse = JoinResponse::default();
}
impl ::pb_jelly::Message for JoinResponse {
  fn compute_size(&self) -> usize  {
    let mut size = 0;
    let mut initial_chats_size = 0;
    for val in &self.initial_chats {
      let l = ::pb_jelly::Message::compute_size(val);
      initial_chats_size += ::pb_jelly::wire_format::serialized_length(1);
      initial_chats_size += ::pb_jelly::varint::serialized_length(l as u64);
      initial_chats_size += l;
    }
    size += initial_chats_size;
    size
  }
  fn compute_grpc_slices_size(&self) -> usize  {
    let mut size = 0;
    for val in &self.initial_chats {
      size += ::pb_jelly::Message::compute_grpc_slices_size(val);
    }
    size
  }
  fn serialize<W: ::pb_jelly::PbBufferWriter>(&self, w: &mut W) -> ::std::io::Result<()> {
    for val in &self.initial_chats {
      ::pb_jelly::wire_format::write(1, ::pb_jelly::wire_format::Type::LengthDelimited, w)?;
      let l = ::pb_jelly::Message::compute_size(val);
      ::pb_jelly::varint::write(l as u64, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    Ok(())
  }
  fn deserialize<B: ::pb_jelly::PbBufferReader>(&mut self, mut buf: &mut B) -> ::std::io::Result<()> {
    while let Some((field_number, typ)) = ::pb_jelly::wire_format::read(&mut buf)? {
      match field_number {
        1 => {
          ::pb_jelly::ensure_wire_format(typ, ::pb_jelly::wire_format::Type::LengthDelimited, "JoinResponse", 1)?;
          let len = ::pb_jelly::varint::ensure_read(&mut buf)?;
          let mut next = ::pb_jelly::ensure_split(buf, len as usize)?;
          let mut val: ::std::string::String = ::std::default::Default::default();
          ::pb_jelly::Message::deserialize(&mut val, &mut next)?;
          self.initial_chats.push(val);
        }
        _ => {
          ::pb_jelly::skip(typ, &mut buf)?;
        }
      }
    }
    Ok(())
  }
}
impl ::pb_jelly::MessageDescriptor for JoinResponse {
  const NAME: &'static str = "JoinResponse";
  const FULL_NAME: &'static str = "chatroom.JoinResponse";
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct SendChat {
  pub msg: ::std::string::String,
}
impl ::std::default::Default for SendChat {
  fn default() -> Self {
    SendChat {
      msg: ::std::default::Default::default(),
    }
  }
}
lazy_static! {
  pub static ref SendChat_default: SendChat = SendChat::default();
}
impl ::pb_jelly::Message for SendChat {
  fn compute_size(&self) -> usize  {
    let mut size = 0;
    let mut msg_size = 0;
    if self.msg != <::std::string::String as ::std::default::Default>::default() {
      let val = &self.msg;
      let l = ::pb_jelly::Message::compute_size(val);
      msg_size += ::pb_jelly::wire_format::serialized_length(1);
      msg_size += ::pb_jelly::varint::serialized_length(l as u64);
      msg_size += l;
    }
    size += msg_size;
    size
  }
  fn compute_grpc_slices_size(&self) -> usize  {
    let mut size = 0;
    if self.msg != <::std::string::String as ::std::default::Default>::default() {
      let val = &self.msg;
      size += ::pb_jelly::Message::compute_grpc_slices_size(val);
    }
    size
  }
  fn serialize<W: ::pb_jelly::PbBufferWriter>(&self, w: &mut W) -> ::std::io::Result<()> {
    if self.msg != <::std::string::String as ::std::default::Default>::default() {
      let val = &self.msg;
      ::pb_jelly::wire_format::write(1, ::pb_jelly::wire_format::Type::LengthDelimited, w)?;
      let l = ::pb_jelly::Message::compute_size(val);
      ::pb_jelly::varint::write(l as u64, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    Ok(())
  }
  fn deserialize<B: ::pb_jelly::PbBufferReader>(&mut self, mut buf: &mut B) -> ::std::io::Result<()> {
    while let Some((field_number, typ)) = ::pb_jelly::wire_format::read(&mut buf)? {
      match field_number {
        1 => {
          ::pb_jelly::ensure_wire_format(typ, ::pb_jelly::wire_format::Type::LengthDelimited, "SendChat", 1)?;
          let len = ::pb_jelly::varint::ensure_read(&mut buf)?;
          let mut next = ::pb_jelly::ensure_split(buf, len as usize)?;
          let mut val: ::std::string::String = ::std::default::Default::default();
          ::pb_jelly::Message::deserialize(&mut val, &mut next)?;
          self.msg = val;
        }
        _ => {
          ::pb_jelly::skip(typ, &mut buf)?;
        }
      }
    }
    Ok(())
  }
}
impl ::pb_jelly::MessageDescriptor for SendChat {
  const NAME: &'static str = "SendChat";
  const FULL_NAME: &'static str = "chatroom.SendChat";
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct RecvChat {
  pub msg: ::std::string::String,
  pub username: ::std::string::String,
}
impl ::std::default::Default for RecvChat {
  fn default() -> Self {
    RecvChat {
      msg: ::std::default::Default::default(),
      username: ::std::default::Default::default(),
    }
  }
}
lazy_static! {
  pub static ref RecvChat_default: RecvChat = RecvChat::default();
}
impl ::pb_jelly::Message for RecvChat {
  fn compute_size(&self) -> usize  {
    let mut size = 0;
    let mut msg_size = 0;
    if self.msg != <::std::string::String as ::std::default::Default>::default() {
      let val = &self.msg;
      let l = ::pb_jelly::Message::compute_size(val);
      msg_size += ::pb_jelly::wire_format::serialized_length(1);
      msg_size += ::pb_jelly::varint::serialized_length(l as u64);
      msg_size += l;
    }
    size += msg_size;
    let mut username_size = 0;
    if self.username != <::std::string::String as ::std::default::Default>::default() {
      let val = &self.username;
      let l = ::pb_jelly::Message::compute_size(val);
      username_size += ::pb_jelly::wire_format::serialized_length(2);
      username_size += ::pb_jelly::varint::serialized_length(l as u64);
      username_size += l;
    }
    size += username_size;
    size
  }
  fn compute_grpc_slices_size(&self) -> usize  {
    let mut size = 0;
    if self.msg != <::std::string::String as ::std::default::Default>::default() {
      let val = &self.msg;
      size += ::pb_jelly::Message::compute_grpc_slices_size(val);
    }
    if self.username != <::std::string::String as ::std::default::Default>::default() {
      let val = &self.username;
      size += ::pb_jelly::Message::compute_grpc_slices_size(val);
    }
    size
  }
  fn serialize<W: ::pb_jelly::PbBufferWriter>(&self, w: &mut W) -> ::std::io::Result<()> {
    if self.msg != <::std::string::String as ::std::default::Default>::default() {
      let val = &self.msg;
      ::pb_jelly::wire_format::write(1, ::pb_jelly::wire_format::Type::LengthDelimited, w)?;
      let l = ::pb_jelly::Message::compute_size(val);
      ::pb_jelly::varint::write(l as u64, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if self.username != <::std::string::String as ::std::default::Default>::default() {
      let val = &self.username;
      ::pb_jelly::wire_format::write(2, ::pb_jelly::wire_format::Type::LengthDelimited, w)?;
      let l = ::pb_jelly::Message::compute_size(val);
      ::pb_jelly::varint::write(l as u64, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    Ok(())
  }
  fn deserialize<B: ::pb_jelly::PbBufferReader>(&mut self, mut buf: &mut B) -> ::std::io::Result<()> {
    while let Some((field_number, typ)) = ::pb_jelly::wire_format::read(&mut buf)? {
      match field_number {
        1 => {
          ::pb_jelly::ensure_wire_format(typ, ::pb_jelly::wire_format::Type::LengthDelimited, "RecvChat", 1)?;
          let len = ::pb_jelly::varint::ensure_read(&mut buf)?;
          let mut next = ::pb_jelly::ensure_split(buf, len as usize)?;
          let mut val: ::std::string::String = ::std::default::Default::default();
          ::pb_jelly::Message::deserialize(&mut val, &mut next)?;
          self.msg = val;
        }
        2 => {
          ::pb_jelly::ensure_wire_format(typ, ::pb_jelly::wire_format::Type::LengthDelimited, "RecvChat", 2)?;
          let len = ::pb_jelly::varint::ensure_read(&mut buf)?;
          let mut next = ::pb_jelly::ensure_split(buf, len as usize)?;
          let mut val: ::std::string::String = ::std::default::Default::default();
          ::pb_jelly::Message::deserialize(&mut val, &mut next)?;
          self.username = val;
        }
        _ => {
          ::pb_jelly::skip(typ, &mut buf)?;
        }
      }
    }
    Ok(())
  }
}
impl ::pb_jelly::MessageDescriptor for RecvChat {
  const NAME: &'static str = "RecvChat";
  const FULL_NAME: &'static str = "chatroom.RecvChat";
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct UserJoin {
  pub username: ::std::string::String,
}
impl ::std::default::Default for UserJoin {
  fn default() -> Self {
    UserJoin {
      username: ::std::default::Default::default(),
    }
  }
}
lazy_static! {
  pub static ref UserJoin_default: UserJoin = UserJoin::default();
}
impl ::pb_jelly::Message for UserJoin {
  fn compute_size(&self) -> usize  {
    let mut size = 0;
    let mut username_size = 0;
    if self.username != <::std::string::String as ::std::default::Default>::default() {
      let val = &self.username;
      let l = ::pb_jelly::Message::compute_size(val);
      username_size += ::pb_jelly::wire_format::serialized_length(1);
      username_size += ::pb_jelly::varint::serialized_length(l as u64);
      username_size += l;
    }
    size += username_size;
    size
  }
  fn compute_grpc_slices_size(&self) -> usize  {
    let mut size = 0;
    if self.username != <::std::string::String as ::std::default::Default>::default() {
      let val = &self.username;
      size += ::pb_jelly::Message::compute_grpc_slices_size(val);
    }
    size
  }
  fn serialize<W: ::pb_jelly::PbBufferWriter>(&self, w: &mut W) -> ::std::io::Result<()> {
    if self.username != <::std::string::String as ::std::default::Default>::default() {
      let val = &self.username;
      ::pb_jelly::wire_format::write(1, ::pb_jelly::wire_format::Type::LengthDelimited, w)?;
      let l = ::pb_jelly::Message::compute_size(val);
      ::pb_jelly::varint::write(l as u64, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    Ok(())
  }
  fn deserialize<B: ::pb_jelly::PbBufferReader>(&mut self, mut buf: &mut B) -> ::std::io::Result<()> {
    while let Some((field_number, typ)) = ::pb_jelly::wire_format::read(&mut buf)? {
      match field_number {
        1 => {
          ::pb_jelly::ensure_wire_format(typ, ::pb_jelly::wire_format::Type::LengthDelimited, "UserJoin", 1)?;
          let len = ::pb_jelly::varint::ensure_read(&mut buf)?;
          let mut next = ::pb_jelly::ensure_split(buf, len as usize)?;
          let mut val: ::std::string::String = ::std::default::Default::default();
          ::pb_jelly::Message::deserialize(&mut val, &mut next)?;
          self.username = val;
        }
        _ => {
          ::pb_jelly::skip(typ, &mut buf)?;
        }
      }
    }
    Ok(())
  }
}
impl ::pb_jelly::MessageDescriptor for UserJoin {
  const NAME: &'static str = "UserJoin";
  const FULL_NAME: &'static str = "chatroom.UserJoin";
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct UserLeft {
  pub username: ::std::string::String,
}
impl ::std::default::Default for UserLeft {
  fn default() -> Self {
    UserLeft {
      username: ::std::default::Default::default(),
    }
  }
}
lazy_static! {
  pub static ref UserLeft_default: UserLeft = UserLeft::default();
}
impl ::pb_jelly::Message for UserLeft {
  fn compute_size(&self) -> usize  {
    let mut size = 0;
    let mut username_size = 0;
    if self.username != <::std::string::String as ::std::default::Default>::default() {
      let val = &self.username;
      let l = ::pb_jelly::Message::compute_size(val);
      username_size += ::pb_jelly::wire_format::serialized_length(1);
      username_size += ::pb_jelly::varint::serialized_length(l as u64);
      username_size += l;
    }
    size += username_size;
    size
  }
  fn compute_grpc_slices_size(&self) -> usize  {
    let mut size = 0;
    if self.username != <::std::string::String as ::std::default::Default>::default() {
      let val = &self.username;
      size += ::pb_jelly::Message::compute_grpc_slices_size(val);
    }
    size
  }
  fn serialize<W: ::pb_jelly::PbBufferWriter>(&self, w: &mut W) -> ::std::io::Result<()> {
    if self.username != <::std::string::String as ::std::default::Default>::default() {
      let val = &self.username;
      ::pb_jelly::wire_format::write(1, ::pb_jelly::wire_format::Type::LengthDelimited, w)?;
      let l = ::pb_jelly::Message::compute_size(val);
      ::pb_jelly::varint::write(l as u64, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    Ok(())
  }
  fn deserialize<B: ::pb_jelly::PbBufferReader>(&mut self, mut buf: &mut B) -> ::std::io::Result<()> {
    while let Some((field_number, typ)) = ::pb_jelly::wire_format::read(&mut buf)? {
      match field_number {
        1 => {
          ::pb_jelly::ensure_wire_format(typ, ::pb_jelly::wire_format::Type::LengthDelimited, "UserLeft", 1)?;
          let len = ::pb_jelly::varint::ensure_read(&mut buf)?;
          let mut next = ::pb_jelly::ensure_split(buf, len as usize)?;
          let mut val: ::std::string::String = ::std::default::Default::default();
          ::pb_jelly::Message::deserialize(&mut val, &mut next)?;
          self.username = val;
        }
        _ => {
          ::pb_jelly::skip(typ, &mut buf)?;
        }
      }
    }
    Ok(())
  }
}
impl ::pb_jelly::MessageDescriptor for UserLeft {
  const NAME: &'static str = "UserLeft";
  const FULL_NAME: &'static str = "chatroom.UserLeft";
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ClientMessageWrapper {
  pub inner_message: ::std::option::Option<ClientMessageWrapper_InnerMessage>,
}
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ClientMessageWrapper_InnerMessage {
  JoinMsg(JoinRequest),
  ChatMsg(SendChat),
}
impl ::std::default::Default for ClientMessageWrapper {
  fn default() -> Self {
    ClientMessageWrapper {
      inner_message: None,
    }
  }
}
lazy_static! {
  pub static ref ClientMessageWrapper_default: ClientMessageWrapper = ClientMessageWrapper::default();
}
impl ::pb_jelly::Message for ClientMessageWrapper {
  fn compute_size(&self) -> usize  {
    let mut size = 0;
    let mut join_msg_size = 0;
    if let Some(ClientMessageWrapper_InnerMessage::JoinMsg(ref val)) = self.inner_message {
      let l = ::pb_jelly::Message::compute_size(val);
      join_msg_size += ::pb_jelly::wire_format::serialized_length(1);
      join_msg_size += ::pb_jelly::varint::serialized_length(l as u64);
      join_msg_size += l;
    }
    size += join_msg_size;
    let mut chat_msg_size = 0;
    if let Some(ClientMessageWrapper_InnerMessage::ChatMsg(ref val)) = self.inner_message {
      let l = ::pb_jelly::Message::compute_size(val);
      chat_msg_size += ::pb_jelly::wire_format::serialized_length(2);
      chat_msg_size += ::pb_jelly::varint::serialized_length(l as u64);
      chat_msg_size += l;
    }
    size += chat_msg_size;
    size
  }
  fn compute_grpc_slices_size(&self) -> usize  {
    let mut size = 0;
    if let Some(ClientMessageWrapper_InnerMessage::JoinMsg(ref val)) = self.inner_message {
      size += ::pb_jelly::Message::compute_grpc_slices_size(val);
    }
    if let Some(ClientMessageWrapper_InnerMessage::ChatMsg(ref val)) = self.inner_message {
      size += ::pb_jelly::Message::compute_grpc_slices_size(val);
    }
    size
  }
  fn serialize<W: ::pb_jelly::PbBufferWriter>(&self, w: &mut W) -> ::std::io::Result<()> {
    if let Some(ClientMessageWrapper_InnerMessage::JoinMsg(ref val)) = self.inner_message {
      ::pb_jelly::wire_format::write(1, ::pb_jelly::wire_format::Type::LengthDelimited, w)?;
      let l = ::pb_jelly::Message::compute_size(val);
      ::pb_jelly::varint::write(l as u64, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if let Some(ClientMessageWrapper_InnerMessage::ChatMsg(ref val)) = self.inner_message {
      ::pb_jelly::wire_format::write(2, ::pb_jelly::wire_format::Type::LengthDelimited, w)?;
      let l = ::pb_jelly::Message::compute_size(val);
      ::pb_jelly::varint::write(l as u64, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    Ok(())
  }
  fn deserialize<B: ::pb_jelly::PbBufferReader>(&mut self, mut buf: &mut B) -> ::std::io::Result<()> {
    while let Some((field_number, typ)) = ::pb_jelly::wire_format::read(&mut buf)? {
      match field_number {
        1 => {
          ::pb_jelly::ensure_wire_format(typ, ::pb_jelly::wire_format::Type::LengthDelimited, "ClientMessageWrapper", 1)?;
          let len = ::pb_jelly::varint::ensure_read(&mut buf)?;
          let mut next = ::pb_jelly::ensure_split(buf, len as usize)?;
          let mut val: JoinRequest = ::std::default::Default::default();
          ::pb_jelly::Message::deserialize(&mut val, &mut next)?;
          self.inner_message = Some(ClientMessageWrapper_InnerMessage::JoinMsg(val));
        }
        2 => {
          ::pb_jelly::ensure_wire_format(typ, ::pb_jelly::wire_format::Type::LengthDelimited, "ClientMessageWrapper", 2)?;
          let len = ::pb_jelly::varint::ensure_read(&mut buf)?;
          let mut next = ::pb_jelly::ensure_split(buf, len as usize)?;
          let mut val: SendChat = ::std::default::Default::default();
          ::pb_jelly::Message::deserialize(&mut val, &mut next)?;
          self.inner_message = Some(ClientMessageWrapper_InnerMessage::ChatMsg(val));
        }
        _ => {
          ::pb_jelly::skip(typ, &mut buf)?;
        }
      }
    }
    Ok(())
  }
}
impl ::pb_jelly::MessageDescriptor for ClientMessageWrapper {
  const NAME: &'static str = "ClientMessageWrapper";
  const FULL_NAME: &'static str = "chatroom.ClientMessageWrapper";
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ServerMessageWrapper {
  pub inner_message: ::std::option::Option<ServerMessageWrapper_InnerMessage>,
}
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ServerMessageWrapper_InnerMessage {
  JoinRespMsg(JoinResponse),
  ChatMsg(RecvChat),
  UserJoinMsg(UserJoin),
  UserLeftMsg(UserLeft),
}
impl ::std::default::Default for ServerMessageWrapper {
  fn default() -> Self {
    ServerMessageWrapper {
      inner_message: None,
    }
  }
}
lazy_static! {
  pub static ref ServerMessageWrapper_default: ServerMessageWrapper = ServerMessageWrapper::default();
}
impl ::pb_jelly::Message for ServerMessageWrapper {
  fn compute_size(&self) -> usize  {
    let mut size = 0;
    let mut join_resp_msg_size = 0;
    if let Some(ServerMessageWrapper_InnerMessage::JoinRespMsg(ref val)) = self.inner_message {
      let l = ::pb_jelly::Message::compute_size(val);
      join_resp_msg_size += ::pb_jelly::wire_format::serialized_length(1);
      join_resp_msg_size += ::pb_jelly::varint::serialized_length(l as u64);
      join_resp_msg_size += l;
    }
    size += join_resp_msg_size;
    let mut chat_msg_size = 0;
    if let Some(ServerMessageWrapper_InnerMessage::ChatMsg(ref val)) = self.inner_message {
      let l = ::pb_jelly::Message::compute_size(val);
      chat_msg_size += ::pb_jelly::wire_format::serialized_length(2);
      chat_msg_size += ::pb_jelly::varint::serialized_length(l as u64);
      chat_msg_size += l;
    }
    size += chat_msg_size;
    let mut user_join_msg_size = 0;
    if let Some(ServerMessageWrapper_InnerMessage::UserJoinMsg(ref val)) = self.inner_message {
      let l = ::pb_jelly::Message::compute_size(val);
      user_join_msg_size += ::pb_jelly::wire_format::serialized_length(3);
      user_join_msg_size += ::pb_jelly::varint::serialized_length(l as u64);
      user_join_msg_size += l;
    }
    size += user_join_msg_size;
    let mut user_left_msg_size = 0;
    if let Some(ServerMessageWrapper_InnerMessage::UserLeftMsg(ref val)) = self.inner_message {
      let l = ::pb_jelly::Message::compute_size(val);
      user_left_msg_size += ::pb_jelly::wire_format::serialized_length(4);
      user_left_msg_size += ::pb_jelly::varint::serialized_length(l as u64);
      user_left_msg_size += l;
    }
    size += user_left_msg_size;
    size
  }
  fn compute_grpc_slices_size(&self) -> usize  {
    let mut size = 0;
    if let Some(ServerMessageWrapper_InnerMessage::JoinRespMsg(ref val)) = self.inner_message {
      size += ::pb_jelly::Message::compute_grpc_slices_size(val);
    }
    if let Some(ServerMessageWrapper_InnerMessage::ChatMsg(ref val)) = self.inner_message {
      size += ::pb_jelly::Message::compute_grpc_slices_size(val);
    }
    if let Some(ServerMessageWrapper_InnerMessage::UserJoinMsg(ref val)) = self.inner_message {
      size += ::pb_jelly::Message::compute_grpc_slices_size(val);
    }
    if let Some(ServerMessageWrapper_InnerMessage::UserLeftMsg(ref val)) = self.inner_message {
      size += ::pb_jelly::Message::compute_grpc_slices_size(val);
    }
    size
  }
  fn serialize<W: ::pb_jelly::PbBufferWriter>(&self, w: &mut W) -> ::std::io::Result<()> {
    if let Some(ServerMessageWrapper_InnerMessage::JoinRespMsg(ref val)) = self.inner_message {
      ::pb_jelly::wire_format::write(1, ::pb_jelly::wire_format::Type::LengthDelimited, w)?;
      let l = ::pb_jelly::Message::compute_size(val);
      ::pb_jelly::varint::write(l as u64, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if let Some(ServerMessageWrapper_InnerMessage::ChatMsg(ref val)) = self.inner_message {
      ::pb_jelly::wire_format::write(2, ::pb_jelly::wire_format::Type::LengthDelimited, w)?;
      let l = ::pb_jelly::Message::compute_size(val);
      ::pb_jelly::varint::write(l as u64, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if let Some(ServerMessageWrapper_InnerMessage::UserJoinMsg(ref val)) = self.inner_message {
      ::pb_jelly::wire_format::write(3, ::pb_jelly::wire_format::Type::LengthDelimited, w)?;
      let l = ::pb_jelly::Message::compute_size(val);
      ::pb_jelly::varint::write(l as u64, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if let Some(ServerMessageWrapper_InnerMessage::UserLeftMsg(ref val)) = self.inner_message {
      ::pb_jelly::wire_format::write(4, ::pb_jelly::wire_format::Type::LengthDelimited, w)?;
      let l = ::pb_jelly::Message::compute_size(val);
      ::pb_jelly::varint::write(l as u64, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    Ok(())
  }
  fn deserialize<B: ::pb_jelly::PbBufferReader>(&mut self, mut buf: &mut B) -> ::std::io::Result<()> {
    while let Some((field_number, typ)) = ::pb_jelly::wire_format::read(&mut buf)? {
      match field_number {
        1 => {
          ::pb_jelly::ensure_wire_format(typ, ::pb_jelly::wire_format::Type::LengthDelimited, "ServerMessageWrapper", 1)?;
          let len = ::pb_jelly::varint::ensure_read(&mut buf)?;
          let mut next = ::pb_jelly::ensure_split(buf, len as usize)?;
          let mut val: JoinResponse = ::std::default::Default::default();
          ::pb_jelly::Message::deserialize(&mut val, &mut next)?;
          self.inner_message = Some(ServerMessageWrapper_InnerMessage::JoinRespMsg(val));
        }
        2 => {
          ::pb_jelly::ensure_wire_format(typ, ::pb_jelly::wire_format::Type::LengthDelimited, "ServerMessageWrapper", 2)?;
          let len = ::pb_jelly::varint::ensure_read(&mut buf)?;
          let mut next = ::pb_jelly::ensure_split(buf, len as usize)?;
          let mut val: RecvChat = ::std::default::Default::default();
          ::pb_jelly::Message::deserialize(&mut val, &mut next)?;
          self.inner_message = Some(ServerMessageWrapper_InnerMessage::ChatMsg(val));
        }
        3 => {
          ::pb_jelly::ensure_wire_format(typ, ::pb_jelly::wire_format::Type::LengthDelimited, "ServerMessageWrapper", 3)?;
          let len = ::pb_jelly::varint::ensure_read(&mut buf)?;
          let mut next = ::pb_jelly::ensure_split(buf, len as usize)?;
          let mut val: UserJoin = ::std::default::Default::default();
          ::pb_jelly::Message::deserialize(&mut val, &mut next)?;
          self.inner_message = Some(ServerMessageWrapper_InnerMessage::UserJoinMsg(val));
        }
        4 => {
          ::pb_jelly::ensure_wire_format(typ, ::pb_jelly::wire_format::Type::LengthDelimited, "ServerMessageWrapper", 4)?;
          let len = ::pb_jelly::varint::ensure_read(&mut buf)?;
          let mut next = ::pb_jelly::ensure_split(buf, len as usize)?;
          let mut val: UserLeft = ::std::default::Default::default();
          ::pb_jelly::Message::deserialize(&mut val, &mut next)?;
          self.inner_message = Some(ServerMessageWrapper_InnerMessage::UserLeftMsg(val));
        }
        _ => {
          ::pb_jelly::skip(typ, &mut buf)?;
        }
      }
    }
    Ok(())
  }
}
impl ::pb_jelly::MessageDescriptor for ServerMessageWrapper {
  const NAME: &'static str = "ServerMessageWrapper";
  const FULL_NAME: &'static str = "chatroom.ServerMessageWrapper";
}

