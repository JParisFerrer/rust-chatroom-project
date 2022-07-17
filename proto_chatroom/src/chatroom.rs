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
  fn descriptor(&self) -> ::std::option::Option<::pb_jelly::MessageDescriptor> {
    Some(::pb_jelly::MessageDescriptor {
      name: "JoinRequest",
      full_name: "chatroom.JoinRequest",
      fields: &[
        ::pb_jelly::FieldDescriptor {
          name: "username",
          full_name: "chatroom.JoinRequest.username",
          index: 0,
          number: 1,
          typ: ::pb_jelly::wire_format::Type::LengthDelimited,
          label: ::pb_jelly::Label::Optional,
          oneof_index: None,
        },
      ],
      oneofs: &[
      ],
    })
  }
  fn compute_size(&self) -> usize {
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
  fn compute_grpc_slices_size(&self) -> usize {
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
impl ::pb_jelly::Reflection for JoinRequest {
  fn which_one_of(&self, oneof_name: &str) -> ::std::option::Option<&'static str> {
    match oneof_name {
      _ => {
        panic!("unknown oneof name given");
      }
    }
  }
  fn get_field_mut(&mut self, field_name: &str) -> ::pb_jelly::reflection::FieldMut<'_> {
    match field_name {
      "username" => {
        ::pb_jelly::reflection::FieldMut::Value(&mut self.username)
      }
      _ => {
        panic!("unknown field name given")
      }
    }
  }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct JoinResponse {
  pub response: JoinResponse_Response,
}
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum JoinResponse_Response {
  Success(JoinResponse_Success),
  Failure(JoinResponse_Failure),
}
impl ::std::default::Default for JoinResponse {
  fn default() -> Self {
    JoinResponse {
      response: JoinResponse_Response::Success(::std::default::Default::default()),
    }
  }
}
lazy_static! {
  pub static ref JoinResponse_default: JoinResponse = JoinResponse::default();
}
impl ::pb_jelly::Message for JoinResponse {
  fn descriptor(&self) -> ::std::option::Option<::pb_jelly::MessageDescriptor> {
    Some(::pb_jelly::MessageDescriptor {
      name: "JoinResponse",
      full_name: "chatroom.JoinResponse",
      fields: &[
        ::pb_jelly::FieldDescriptor {
          name: "success",
          full_name: "chatroom.JoinResponse.success",
          index: 0,
          number: 1,
          typ: ::pb_jelly::wire_format::Type::LengthDelimited,
          label: ::pb_jelly::Label::Optional,
          oneof_index: Some(0),
        },
        ::pb_jelly::FieldDescriptor {
          name: "failure",
          full_name: "chatroom.JoinResponse.failure",
          index: 1,
          number: 2,
          typ: ::pb_jelly::wire_format::Type::LengthDelimited,
          label: ::pb_jelly::Label::Optional,
          oneof_index: Some(0),
        },
      ],
      oneofs: &[
        ::pb_jelly::OneofDescriptor {
          name: "response",
        },
      ],
    })
  }
  fn compute_size(&self) -> usize {
    let mut size = 0;
    let mut success_size = 0;
    if let JoinResponse_Response::Success(ref val) = self.response {
      let l = ::pb_jelly::Message::compute_size(val);
      success_size += ::pb_jelly::wire_format::serialized_length(1);
      success_size += ::pb_jelly::varint::serialized_length(l as u64);
      success_size += l;
    }
    size += success_size;
    let mut failure_size = 0;
    if let JoinResponse_Response::Failure(ref val) = self.response {
      let l = ::pb_jelly::Message::compute_size(val);
      failure_size += ::pb_jelly::wire_format::serialized_length(2);
      failure_size += ::pb_jelly::varint::serialized_length(l as u64);
      failure_size += l;
    }
    size += failure_size;
    size
  }
  fn compute_grpc_slices_size(&self) -> usize {
    let mut size = 0;
    if let JoinResponse_Response::Success(ref val) = self.response {
      size += ::pb_jelly::Message::compute_grpc_slices_size(val);
    }
    if let JoinResponse_Response::Failure(ref val) = self.response {
      size += ::pb_jelly::Message::compute_grpc_slices_size(val);
    }
    size
  }
  fn serialize<W: ::pb_jelly::PbBufferWriter>(&self, w: &mut W) -> ::std::io::Result<()> {
    if let JoinResponse_Response::Success(ref val) = self.response {
      ::pb_jelly::wire_format::write(1, ::pb_jelly::wire_format::Type::LengthDelimited, w)?;
      let l = ::pb_jelly::Message::compute_size(val);
      ::pb_jelly::varint::write(l as u64, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if let JoinResponse_Response::Failure(ref val) = self.response {
      ::pb_jelly::wire_format::write(2, ::pb_jelly::wire_format::Type::LengthDelimited, w)?;
      let l = ::pb_jelly::Message::compute_size(val);
      ::pb_jelly::varint::write(l as u64, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    Ok(())
  }
  fn deserialize<B: ::pb_jelly::PbBufferReader>(&mut self, mut buf: &mut B) -> ::std::io::Result<()> {
    let mut oneof_response: ::std::option::Option<JoinResponse_Response> = None;
    while let Some((field_number, typ)) = ::pb_jelly::wire_format::read(&mut buf)? {
      match field_number {
        1 => {
          ::pb_jelly::ensure_wire_format(typ, ::pb_jelly::wire_format::Type::LengthDelimited, "JoinResponse", 1)?;
          let len = ::pb_jelly::varint::ensure_read(&mut buf)?;
          let mut next = ::pb_jelly::ensure_split(buf, len as usize)?;
          let mut val: JoinResponse_Success = ::std::default::Default::default();
          ::pb_jelly::Message::deserialize(&mut val, &mut next)?;
          oneof_response = Some(JoinResponse_Response::Success(val));
        }
        2 => {
          ::pb_jelly::ensure_wire_format(typ, ::pb_jelly::wire_format::Type::LengthDelimited, "JoinResponse", 2)?;
          let len = ::pb_jelly::varint::ensure_read(&mut buf)?;
          let mut next = ::pb_jelly::ensure_split(buf, len as usize)?;
          let mut val: JoinResponse_Failure = ::std::default::Default::default();
          ::pb_jelly::Message::deserialize(&mut val, &mut next)?;
          oneof_response = Some(JoinResponse_Response::Failure(val));
        }
        _ => {
          ::pb_jelly::skip(typ, &mut buf)?;
        }
      }
    }
    match oneof_response {
      Some(v) => self.response = v,
      None => return Err(::std::io::Error::new(::std::io::ErrorKind::InvalidInput, "missing value for non-nullable oneof 'response' while parsing message chatroom.JoinResponse")),
    }
    Ok(())
  }
}
impl ::pb_jelly::Reflection for JoinResponse {
  fn which_one_of(&self, oneof_name: &str) -> ::std::option::Option<&'static str> {
    match oneof_name {
      "response" => {
        if let JoinResponse_Response::Success(ref val) = self.response {
          return Some("success");
        }
        if let JoinResponse_Response::Failure(ref val) = self.response {
          return Some("failure");
        }
        None
      }
      _ => {
        panic!("unknown oneof name given");
      }
    }
  }
  fn get_field_mut(&mut self, field_name: &str) -> ::pb_jelly::reflection::FieldMut<'_> {
    match field_name {
      "success" => {
        match self.response {
          JoinResponse_Response::Success(_) => (),
          _ => {
            self.response = JoinResponse_Response::Success(::std::default::Default::default());
          },
        }
        if let JoinResponse_Response::Success(ref mut val) = self.response {
          return ::pb_jelly::reflection::FieldMut::Value(val);
        }
        unreachable!()
      }
      "failure" => {
        match self.response {
          JoinResponse_Response::Failure(_) => (),
          _ => {
            self.response = JoinResponse_Response::Failure(::std::default::Default::default());
          },
        }
        if let JoinResponse_Response::Failure(ref mut val) = self.response {
          return ::pb_jelly::reflection::FieldMut::Value(val);
        }
        unreachable!()
      }
      _ => {
        panic!("unknown field name given")
      }
    }
  }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct JoinResponse_Success {
  /// This should really include leaves & joins
  pub initial_chats: ::std::vec::Vec<RecvChat>,
}
impl ::std::default::Default for JoinResponse_Success {
  fn default() -> Self {
    JoinResponse_Success {
      initial_chats: ::std::default::Default::default(),
    }
  }
}
lazy_static! {
  pub static ref JoinResponse_Success_default: JoinResponse_Success = JoinResponse_Success::default();
}
impl ::pb_jelly::Message for JoinResponse_Success {
  fn descriptor(&self) -> ::std::option::Option<::pb_jelly::MessageDescriptor> {
    Some(::pb_jelly::MessageDescriptor {
      name: "JoinResponse_Success",
      full_name: "chatroom.JoinResponse_Success",
      fields: &[
        ::pb_jelly::FieldDescriptor {
          name: "initial_chats",
          full_name: "chatroom.JoinResponse_Success.initial_chats",
          index: 0,
          number: 1,
          typ: ::pb_jelly::wire_format::Type::LengthDelimited,
          label: ::pb_jelly::Label::Repeated,
          oneof_index: None,
        },
      ],
      oneofs: &[
      ],
    })
  }
  fn compute_size(&self) -> usize {
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
  fn compute_grpc_slices_size(&self) -> usize {
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
          ::pb_jelly::ensure_wire_format(typ, ::pb_jelly::wire_format::Type::LengthDelimited, "JoinResponse_Success", 1)?;
          let len = ::pb_jelly::varint::ensure_read(&mut buf)?;
          let mut next = ::pb_jelly::ensure_split(buf, len as usize)?;
          let mut val: RecvChat = ::std::default::Default::default();
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
impl ::pb_jelly::Reflection for JoinResponse_Success {
  fn which_one_of(&self, oneof_name: &str) -> ::std::option::Option<&'static str> {
    match oneof_name {
      _ => {
        panic!("unknown oneof name given");
      }
    }
  }
  fn get_field_mut(&mut self, field_name: &str) -> ::pb_jelly::reflection::FieldMut<'_> {
    match field_name {
      "initial_chats" => {
        unimplemented!("Repeated fields are not currently supported.")
      }
      _ => {
        panic!("unknown field name given")
      }
    }
  }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct JoinResponse_Failure {
  pub error_message: ::std::string::String,
}
impl ::std::default::Default for JoinResponse_Failure {
  fn default() -> Self {
    JoinResponse_Failure {
      error_message: ::std::default::Default::default(),
    }
  }
}
lazy_static! {
  pub static ref JoinResponse_Failure_default: JoinResponse_Failure = JoinResponse_Failure::default();
}
impl ::pb_jelly::Message for JoinResponse_Failure {
  fn descriptor(&self) -> ::std::option::Option<::pb_jelly::MessageDescriptor> {
    Some(::pb_jelly::MessageDescriptor {
      name: "JoinResponse_Failure",
      full_name: "chatroom.JoinResponse_Failure",
      fields: &[
        ::pb_jelly::FieldDescriptor {
          name: "error_message",
          full_name: "chatroom.JoinResponse_Failure.error_message",
          index: 0,
          number: 1,
          typ: ::pb_jelly::wire_format::Type::LengthDelimited,
          label: ::pb_jelly::Label::Optional,
          oneof_index: None,
        },
      ],
      oneofs: &[
      ],
    })
  }
  fn compute_size(&self) -> usize {
    let mut size = 0;
    let mut error_message_size = 0;
    if self.error_message != <::std::string::String as ::std::default::Default>::default() {
      let val = &self.error_message;
      let l = ::pb_jelly::Message::compute_size(val);
      error_message_size += ::pb_jelly::wire_format::serialized_length(1);
      error_message_size += ::pb_jelly::varint::serialized_length(l as u64);
      error_message_size += l;
    }
    size += error_message_size;
    size
  }
  fn compute_grpc_slices_size(&self) -> usize {
    let mut size = 0;
    if self.error_message != <::std::string::String as ::std::default::Default>::default() {
      let val = &self.error_message;
      size += ::pb_jelly::Message::compute_grpc_slices_size(val);
    }
    size
  }
  fn serialize<W: ::pb_jelly::PbBufferWriter>(&self, w: &mut W) -> ::std::io::Result<()> {
    if self.error_message != <::std::string::String as ::std::default::Default>::default() {
      let val = &self.error_message;
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
          ::pb_jelly::ensure_wire_format(typ, ::pb_jelly::wire_format::Type::LengthDelimited, "JoinResponse_Failure", 1)?;
          let len = ::pb_jelly::varint::ensure_read(&mut buf)?;
          let mut next = ::pb_jelly::ensure_split(buf, len as usize)?;
          let mut val: ::std::string::String = ::std::default::Default::default();
          ::pb_jelly::Message::deserialize(&mut val, &mut next)?;
          self.error_message = val;
        }
        _ => {
          ::pb_jelly::skip(typ, &mut buf)?;
        }
      }
    }
    Ok(())
  }
}
impl ::pb_jelly::Reflection for JoinResponse_Failure {
  fn which_one_of(&self, oneof_name: &str) -> ::std::option::Option<&'static str> {
    match oneof_name {
      _ => {
        panic!("unknown oneof name given");
      }
    }
  }
  fn get_field_mut(&mut self, field_name: &str) -> ::pb_jelly::reflection::FieldMut<'_> {
    match field_name {
      "error_message" => {
        ::pb_jelly::reflection::FieldMut::Value(&mut self.error_message)
      }
      _ => {
        panic!("unknown field name given")
      }
    }
  }
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
  fn descriptor(&self) -> ::std::option::Option<::pb_jelly::MessageDescriptor> {
    Some(::pb_jelly::MessageDescriptor {
      name: "SendChat",
      full_name: "chatroom.SendChat",
      fields: &[
        ::pb_jelly::FieldDescriptor {
          name: "msg",
          full_name: "chatroom.SendChat.msg",
          index: 0,
          number: 1,
          typ: ::pb_jelly::wire_format::Type::LengthDelimited,
          label: ::pb_jelly::Label::Optional,
          oneof_index: None,
        },
      ],
      oneofs: &[
      ],
    })
  }
  fn compute_size(&self) -> usize {
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
  fn compute_grpc_slices_size(&self) -> usize {
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
impl ::pb_jelly::Reflection for SendChat {
  fn which_one_of(&self, oneof_name: &str) -> ::std::option::Option<&'static str> {
    match oneof_name {
      _ => {
        panic!("unknown oneof name given");
      }
    }
  }
  fn get_field_mut(&mut self, field_name: &str) -> ::pb_jelly::reflection::FieldMut<'_> {
    match field_name {
      "msg" => {
        ::pb_jelly::reflection::FieldMut::Value(&mut self.msg)
      }
      _ => {
        panic!("unknown field name given")
      }
    }
  }
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
  fn descriptor(&self) -> ::std::option::Option<::pb_jelly::MessageDescriptor> {
    Some(::pb_jelly::MessageDescriptor {
      name: "RecvChat",
      full_name: "chatroom.RecvChat",
      fields: &[
        ::pb_jelly::FieldDescriptor {
          name: "msg",
          full_name: "chatroom.RecvChat.msg",
          index: 0,
          number: 1,
          typ: ::pb_jelly::wire_format::Type::LengthDelimited,
          label: ::pb_jelly::Label::Optional,
          oneof_index: None,
        },
        ::pb_jelly::FieldDescriptor {
          name: "username",
          full_name: "chatroom.RecvChat.username",
          index: 1,
          number: 2,
          typ: ::pb_jelly::wire_format::Type::LengthDelimited,
          label: ::pb_jelly::Label::Optional,
          oneof_index: None,
        },
      ],
      oneofs: &[
      ],
    })
  }
  fn compute_size(&self) -> usize {
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
  fn compute_grpc_slices_size(&self) -> usize {
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
impl ::pb_jelly::Reflection for RecvChat {
  fn which_one_of(&self, oneof_name: &str) -> ::std::option::Option<&'static str> {
    match oneof_name {
      _ => {
        panic!("unknown oneof name given");
      }
    }
  }
  fn get_field_mut(&mut self, field_name: &str) -> ::pb_jelly::reflection::FieldMut<'_> {
    match field_name {
      "msg" => {
        ::pb_jelly::reflection::FieldMut::Value(&mut self.msg)
      }
      "username" => {
        ::pb_jelly::reflection::FieldMut::Value(&mut self.username)
      }
      _ => {
        panic!("unknown field name given")
      }
    }
  }
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
  fn descriptor(&self) -> ::std::option::Option<::pb_jelly::MessageDescriptor> {
    Some(::pb_jelly::MessageDescriptor {
      name: "UserJoin",
      full_name: "chatroom.UserJoin",
      fields: &[
        ::pb_jelly::FieldDescriptor {
          name: "username",
          full_name: "chatroom.UserJoin.username",
          index: 0,
          number: 1,
          typ: ::pb_jelly::wire_format::Type::LengthDelimited,
          label: ::pb_jelly::Label::Optional,
          oneof_index: None,
        },
      ],
      oneofs: &[
      ],
    })
  }
  fn compute_size(&self) -> usize {
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
  fn compute_grpc_slices_size(&self) -> usize {
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
impl ::pb_jelly::Reflection for UserJoin {
  fn which_one_of(&self, oneof_name: &str) -> ::std::option::Option<&'static str> {
    match oneof_name {
      _ => {
        panic!("unknown oneof name given");
      }
    }
  }
  fn get_field_mut(&mut self, field_name: &str) -> ::pb_jelly::reflection::FieldMut<'_> {
    match field_name {
      "username" => {
        ::pb_jelly::reflection::FieldMut::Value(&mut self.username)
      }
      _ => {
        panic!("unknown field name given")
      }
    }
  }
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
  fn descriptor(&self) -> ::std::option::Option<::pb_jelly::MessageDescriptor> {
    Some(::pb_jelly::MessageDescriptor {
      name: "UserLeft",
      full_name: "chatroom.UserLeft",
      fields: &[
        ::pb_jelly::FieldDescriptor {
          name: "username",
          full_name: "chatroom.UserLeft.username",
          index: 0,
          number: 1,
          typ: ::pb_jelly::wire_format::Type::LengthDelimited,
          label: ::pb_jelly::Label::Optional,
          oneof_index: None,
        },
      ],
      oneofs: &[
      ],
    })
  }
  fn compute_size(&self) -> usize {
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
  fn compute_grpc_slices_size(&self) -> usize {
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
impl ::pb_jelly::Reflection for UserLeft {
  fn which_one_of(&self, oneof_name: &str) -> ::std::option::Option<&'static str> {
    match oneof_name {
      _ => {
        panic!("unknown oneof name given");
      }
    }
  }
  fn get_field_mut(&mut self, field_name: &str) -> ::pb_jelly::reflection::FieldMut<'_> {
    match field_name {
      "username" => {
        ::pb_jelly::reflection::FieldMut::Value(&mut self.username)
      }
      _ => {
        panic!("unknown field name given")
      }
    }
  }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ClientMessageWrapper {
  pub inner_message: ClientMessageWrapper_InnerMessage,
}
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ClientMessageWrapper_InnerMessage {
  JoinMsg(JoinRequest),
  ChatMsg(SendChat),
}
impl ::std::default::Default for ClientMessageWrapper {
  fn default() -> Self {
    ClientMessageWrapper {
      inner_message: ClientMessageWrapper_InnerMessage::JoinMsg(::std::default::Default::default()),
    }
  }
}
lazy_static! {
  pub static ref ClientMessageWrapper_default: ClientMessageWrapper = ClientMessageWrapper::default();
}
impl ::pb_jelly::Message for ClientMessageWrapper {
  fn descriptor(&self) -> ::std::option::Option<::pb_jelly::MessageDescriptor> {
    Some(::pb_jelly::MessageDescriptor {
      name: "ClientMessageWrapper",
      full_name: "chatroom.ClientMessageWrapper",
      fields: &[
        ::pb_jelly::FieldDescriptor {
          name: "join_msg",
          full_name: "chatroom.ClientMessageWrapper.join_msg",
          index: 0,
          number: 1,
          typ: ::pb_jelly::wire_format::Type::LengthDelimited,
          label: ::pb_jelly::Label::Optional,
          oneof_index: Some(0),
        },
        ::pb_jelly::FieldDescriptor {
          name: "chat_msg",
          full_name: "chatroom.ClientMessageWrapper.chat_msg",
          index: 1,
          number: 2,
          typ: ::pb_jelly::wire_format::Type::LengthDelimited,
          label: ::pb_jelly::Label::Optional,
          oneof_index: Some(0),
        },
      ],
      oneofs: &[
        ::pb_jelly::OneofDescriptor {
          name: "inner_message",
        },
      ],
    })
  }
  fn compute_size(&self) -> usize {
    let mut size = 0;
    let mut join_msg_size = 0;
    if let ClientMessageWrapper_InnerMessage::JoinMsg(ref val) = self.inner_message {
      let l = ::pb_jelly::Message::compute_size(val);
      join_msg_size += ::pb_jelly::wire_format::serialized_length(1);
      join_msg_size += ::pb_jelly::varint::serialized_length(l as u64);
      join_msg_size += l;
    }
    size += join_msg_size;
    let mut chat_msg_size = 0;
    if let ClientMessageWrapper_InnerMessage::ChatMsg(ref val) = self.inner_message {
      let l = ::pb_jelly::Message::compute_size(val);
      chat_msg_size += ::pb_jelly::wire_format::serialized_length(2);
      chat_msg_size += ::pb_jelly::varint::serialized_length(l as u64);
      chat_msg_size += l;
    }
    size += chat_msg_size;
    size
  }
  fn compute_grpc_slices_size(&self) -> usize {
    let mut size = 0;
    if let ClientMessageWrapper_InnerMessage::JoinMsg(ref val) = self.inner_message {
      size += ::pb_jelly::Message::compute_grpc_slices_size(val);
    }
    if let ClientMessageWrapper_InnerMessage::ChatMsg(ref val) = self.inner_message {
      size += ::pb_jelly::Message::compute_grpc_slices_size(val);
    }
    size
  }
  fn serialize<W: ::pb_jelly::PbBufferWriter>(&self, w: &mut W) -> ::std::io::Result<()> {
    if let ClientMessageWrapper_InnerMessage::JoinMsg(ref val) = self.inner_message {
      ::pb_jelly::wire_format::write(1, ::pb_jelly::wire_format::Type::LengthDelimited, w)?;
      let l = ::pb_jelly::Message::compute_size(val);
      ::pb_jelly::varint::write(l as u64, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if let ClientMessageWrapper_InnerMessage::ChatMsg(ref val) = self.inner_message {
      ::pb_jelly::wire_format::write(2, ::pb_jelly::wire_format::Type::LengthDelimited, w)?;
      let l = ::pb_jelly::Message::compute_size(val);
      ::pb_jelly::varint::write(l as u64, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    Ok(())
  }
  fn deserialize<B: ::pb_jelly::PbBufferReader>(&mut self, mut buf: &mut B) -> ::std::io::Result<()> {
    let mut oneof_inner_message: ::std::option::Option<ClientMessageWrapper_InnerMessage> = None;
    while let Some((field_number, typ)) = ::pb_jelly::wire_format::read(&mut buf)? {
      match field_number {
        1 => {
          ::pb_jelly::ensure_wire_format(typ, ::pb_jelly::wire_format::Type::LengthDelimited, "ClientMessageWrapper", 1)?;
          let len = ::pb_jelly::varint::ensure_read(&mut buf)?;
          let mut next = ::pb_jelly::ensure_split(buf, len as usize)?;
          let mut val: JoinRequest = ::std::default::Default::default();
          ::pb_jelly::Message::deserialize(&mut val, &mut next)?;
          oneof_inner_message = Some(ClientMessageWrapper_InnerMessage::JoinMsg(val));
        }
        2 => {
          ::pb_jelly::ensure_wire_format(typ, ::pb_jelly::wire_format::Type::LengthDelimited, "ClientMessageWrapper", 2)?;
          let len = ::pb_jelly::varint::ensure_read(&mut buf)?;
          let mut next = ::pb_jelly::ensure_split(buf, len as usize)?;
          let mut val: SendChat = ::std::default::Default::default();
          ::pb_jelly::Message::deserialize(&mut val, &mut next)?;
          oneof_inner_message = Some(ClientMessageWrapper_InnerMessage::ChatMsg(val));
        }
        _ => {
          ::pb_jelly::skip(typ, &mut buf)?;
        }
      }
    }
    match oneof_inner_message {
      Some(v) => self.inner_message = v,
      None => return Err(::std::io::Error::new(::std::io::ErrorKind::InvalidInput, "missing value for non-nullable oneof 'inner_message' while parsing message chatroom.ClientMessageWrapper")),
    }
    Ok(())
  }
}
impl ::pb_jelly::Reflection for ClientMessageWrapper {
  fn which_one_of(&self, oneof_name: &str) -> ::std::option::Option<&'static str> {
    match oneof_name {
      "inner_message" => {
        if let ClientMessageWrapper_InnerMessage::JoinMsg(ref val) = self.inner_message {
          return Some("join_msg");
        }
        if let ClientMessageWrapper_InnerMessage::ChatMsg(ref val) = self.inner_message {
          return Some("chat_msg");
        }
        None
      }
      _ => {
        panic!("unknown oneof name given");
      }
    }
  }
  fn get_field_mut(&mut self, field_name: &str) -> ::pb_jelly::reflection::FieldMut<'_> {
    match field_name {
      "join_msg" => {
        match self.inner_message {
          ClientMessageWrapper_InnerMessage::JoinMsg(_) => (),
          _ => {
            self.inner_message = ClientMessageWrapper_InnerMessage::JoinMsg(::std::default::Default::default());
          },
        }
        if let ClientMessageWrapper_InnerMessage::JoinMsg(ref mut val) = self.inner_message {
          return ::pb_jelly::reflection::FieldMut::Value(val);
        }
        unreachable!()
      }
      "chat_msg" => {
        match self.inner_message {
          ClientMessageWrapper_InnerMessage::ChatMsg(_) => (),
          _ => {
            self.inner_message = ClientMessageWrapper_InnerMessage::ChatMsg(::std::default::Default::default());
          },
        }
        if let ClientMessageWrapper_InnerMessage::ChatMsg(ref mut val) = self.inner_message {
          return ::pb_jelly::reflection::FieldMut::Value(val);
        }
        unreachable!()
      }
      _ => {
        panic!("unknown field name given")
      }
    }
  }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ServerMessageWrapper {
  pub inner_message: ServerMessageWrapper_InnerMessage,
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
      inner_message: ServerMessageWrapper_InnerMessage::JoinRespMsg(::std::default::Default::default()),
    }
  }
}
lazy_static! {
  pub static ref ServerMessageWrapper_default: ServerMessageWrapper = ServerMessageWrapper::default();
}
impl ::pb_jelly::Message for ServerMessageWrapper {
  fn descriptor(&self) -> ::std::option::Option<::pb_jelly::MessageDescriptor> {
    Some(::pb_jelly::MessageDescriptor {
      name: "ServerMessageWrapper",
      full_name: "chatroom.ServerMessageWrapper",
      fields: &[
        ::pb_jelly::FieldDescriptor {
          name: "join_resp_msg",
          full_name: "chatroom.ServerMessageWrapper.join_resp_msg",
          index: 0,
          number: 1,
          typ: ::pb_jelly::wire_format::Type::LengthDelimited,
          label: ::pb_jelly::Label::Optional,
          oneof_index: Some(0),
        },
        ::pb_jelly::FieldDescriptor {
          name: "chat_msg",
          full_name: "chatroom.ServerMessageWrapper.chat_msg",
          index: 1,
          number: 2,
          typ: ::pb_jelly::wire_format::Type::LengthDelimited,
          label: ::pb_jelly::Label::Optional,
          oneof_index: Some(0),
        },
        ::pb_jelly::FieldDescriptor {
          name: "user_join_msg",
          full_name: "chatroom.ServerMessageWrapper.user_join_msg",
          index: 2,
          number: 3,
          typ: ::pb_jelly::wire_format::Type::LengthDelimited,
          label: ::pb_jelly::Label::Optional,
          oneof_index: Some(0),
        },
        ::pb_jelly::FieldDescriptor {
          name: "user_left_msg",
          full_name: "chatroom.ServerMessageWrapper.user_left_msg",
          index: 3,
          number: 4,
          typ: ::pb_jelly::wire_format::Type::LengthDelimited,
          label: ::pb_jelly::Label::Optional,
          oneof_index: Some(0),
        },
      ],
      oneofs: &[
        ::pb_jelly::OneofDescriptor {
          name: "inner_message",
        },
      ],
    })
  }
  fn compute_size(&self) -> usize {
    let mut size = 0;
    let mut join_resp_msg_size = 0;
    if let ServerMessageWrapper_InnerMessage::JoinRespMsg(ref val) = self.inner_message {
      let l = ::pb_jelly::Message::compute_size(val);
      join_resp_msg_size += ::pb_jelly::wire_format::serialized_length(1);
      join_resp_msg_size += ::pb_jelly::varint::serialized_length(l as u64);
      join_resp_msg_size += l;
    }
    size += join_resp_msg_size;
    let mut chat_msg_size = 0;
    if let ServerMessageWrapper_InnerMessage::ChatMsg(ref val) = self.inner_message {
      let l = ::pb_jelly::Message::compute_size(val);
      chat_msg_size += ::pb_jelly::wire_format::serialized_length(2);
      chat_msg_size += ::pb_jelly::varint::serialized_length(l as u64);
      chat_msg_size += l;
    }
    size += chat_msg_size;
    let mut user_join_msg_size = 0;
    if let ServerMessageWrapper_InnerMessage::UserJoinMsg(ref val) = self.inner_message {
      let l = ::pb_jelly::Message::compute_size(val);
      user_join_msg_size += ::pb_jelly::wire_format::serialized_length(3);
      user_join_msg_size += ::pb_jelly::varint::serialized_length(l as u64);
      user_join_msg_size += l;
    }
    size += user_join_msg_size;
    let mut user_left_msg_size = 0;
    if let ServerMessageWrapper_InnerMessage::UserLeftMsg(ref val) = self.inner_message {
      let l = ::pb_jelly::Message::compute_size(val);
      user_left_msg_size += ::pb_jelly::wire_format::serialized_length(4);
      user_left_msg_size += ::pb_jelly::varint::serialized_length(l as u64);
      user_left_msg_size += l;
    }
    size += user_left_msg_size;
    size
  }
  fn compute_grpc_slices_size(&self) -> usize {
    let mut size = 0;
    if let ServerMessageWrapper_InnerMessage::JoinRespMsg(ref val) = self.inner_message {
      size += ::pb_jelly::Message::compute_grpc_slices_size(val);
    }
    if let ServerMessageWrapper_InnerMessage::ChatMsg(ref val) = self.inner_message {
      size += ::pb_jelly::Message::compute_grpc_slices_size(val);
    }
    if let ServerMessageWrapper_InnerMessage::UserJoinMsg(ref val) = self.inner_message {
      size += ::pb_jelly::Message::compute_grpc_slices_size(val);
    }
    if let ServerMessageWrapper_InnerMessage::UserLeftMsg(ref val) = self.inner_message {
      size += ::pb_jelly::Message::compute_grpc_slices_size(val);
    }
    size
  }
  fn serialize<W: ::pb_jelly::PbBufferWriter>(&self, w: &mut W) -> ::std::io::Result<()> {
    if let ServerMessageWrapper_InnerMessage::JoinRespMsg(ref val) = self.inner_message {
      ::pb_jelly::wire_format::write(1, ::pb_jelly::wire_format::Type::LengthDelimited, w)?;
      let l = ::pb_jelly::Message::compute_size(val);
      ::pb_jelly::varint::write(l as u64, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if let ServerMessageWrapper_InnerMessage::ChatMsg(ref val) = self.inner_message {
      ::pb_jelly::wire_format::write(2, ::pb_jelly::wire_format::Type::LengthDelimited, w)?;
      let l = ::pb_jelly::Message::compute_size(val);
      ::pb_jelly::varint::write(l as u64, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if let ServerMessageWrapper_InnerMessage::UserJoinMsg(ref val) = self.inner_message {
      ::pb_jelly::wire_format::write(3, ::pb_jelly::wire_format::Type::LengthDelimited, w)?;
      let l = ::pb_jelly::Message::compute_size(val);
      ::pb_jelly::varint::write(l as u64, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if let ServerMessageWrapper_InnerMessage::UserLeftMsg(ref val) = self.inner_message {
      ::pb_jelly::wire_format::write(4, ::pb_jelly::wire_format::Type::LengthDelimited, w)?;
      let l = ::pb_jelly::Message::compute_size(val);
      ::pb_jelly::varint::write(l as u64, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    Ok(())
  }
  fn deserialize<B: ::pb_jelly::PbBufferReader>(&mut self, mut buf: &mut B) -> ::std::io::Result<()> {
    let mut oneof_inner_message: ::std::option::Option<ServerMessageWrapper_InnerMessage> = None;
    while let Some((field_number, typ)) = ::pb_jelly::wire_format::read(&mut buf)? {
      match field_number {
        1 => {
          ::pb_jelly::ensure_wire_format(typ, ::pb_jelly::wire_format::Type::LengthDelimited, "ServerMessageWrapper", 1)?;
          let len = ::pb_jelly::varint::ensure_read(&mut buf)?;
          let mut next = ::pb_jelly::ensure_split(buf, len as usize)?;
          let mut val: JoinResponse = ::std::default::Default::default();
          ::pb_jelly::Message::deserialize(&mut val, &mut next)?;
          oneof_inner_message = Some(ServerMessageWrapper_InnerMessage::JoinRespMsg(val));
        }
        2 => {
          ::pb_jelly::ensure_wire_format(typ, ::pb_jelly::wire_format::Type::LengthDelimited, "ServerMessageWrapper", 2)?;
          let len = ::pb_jelly::varint::ensure_read(&mut buf)?;
          let mut next = ::pb_jelly::ensure_split(buf, len as usize)?;
          let mut val: RecvChat = ::std::default::Default::default();
          ::pb_jelly::Message::deserialize(&mut val, &mut next)?;
          oneof_inner_message = Some(ServerMessageWrapper_InnerMessage::ChatMsg(val));
        }
        3 => {
          ::pb_jelly::ensure_wire_format(typ, ::pb_jelly::wire_format::Type::LengthDelimited, "ServerMessageWrapper", 3)?;
          let len = ::pb_jelly::varint::ensure_read(&mut buf)?;
          let mut next = ::pb_jelly::ensure_split(buf, len as usize)?;
          let mut val: UserJoin = ::std::default::Default::default();
          ::pb_jelly::Message::deserialize(&mut val, &mut next)?;
          oneof_inner_message = Some(ServerMessageWrapper_InnerMessage::UserJoinMsg(val));
        }
        4 => {
          ::pb_jelly::ensure_wire_format(typ, ::pb_jelly::wire_format::Type::LengthDelimited, "ServerMessageWrapper", 4)?;
          let len = ::pb_jelly::varint::ensure_read(&mut buf)?;
          let mut next = ::pb_jelly::ensure_split(buf, len as usize)?;
          let mut val: UserLeft = ::std::default::Default::default();
          ::pb_jelly::Message::deserialize(&mut val, &mut next)?;
          oneof_inner_message = Some(ServerMessageWrapper_InnerMessage::UserLeftMsg(val));
        }
        _ => {
          ::pb_jelly::skip(typ, &mut buf)?;
        }
      }
    }
    match oneof_inner_message {
      Some(v) => self.inner_message = v,
      None => return Err(::std::io::Error::new(::std::io::ErrorKind::InvalidInput, "missing value for non-nullable oneof 'inner_message' while parsing message chatroom.ServerMessageWrapper")),
    }
    Ok(())
  }
}
impl ::pb_jelly::Reflection for ServerMessageWrapper {
  fn which_one_of(&self, oneof_name: &str) -> ::std::option::Option<&'static str> {
    match oneof_name {
      "inner_message" => {
        if let ServerMessageWrapper_InnerMessage::JoinRespMsg(ref val) = self.inner_message {
          return Some("join_resp_msg");
        }
        if let ServerMessageWrapper_InnerMessage::ChatMsg(ref val) = self.inner_message {
          return Some("chat_msg");
        }
        if let ServerMessageWrapper_InnerMessage::UserJoinMsg(ref val) = self.inner_message {
          return Some("user_join_msg");
        }
        if let ServerMessageWrapper_InnerMessage::UserLeftMsg(ref val) = self.inner_message {
          return Some("user_left_msg");
        }
        None
      }
      _ => {
        panic!("unknown oneof name given");
      }
    }
  }
  fn get_field_mut(&mut self, field_name: &str) -> ::pb_jelly::reflection::FieldMut<'_> {
    match field_name {
      "join_resp_msg" => {
        match self.inner_message {
          ServerMessageWrapper_InnerMessage::JoinRespMsg(_) => (),
          _ => {
            self.inner_message = ServerMessageWrapper_InnerMessage::JoinRespMsg(::std::default::Default::default());
          },
        }
        if let ServerMessageWrapper_InnerMessage::JoinRespMsg(ref mut val) = self.inner_message {
          return ::pb_jelly::reflection::FieldMut::Value(val);
        }
        unreachable!()
      }
      "chat_msg" => {
        match self.inner_message {
          ServerMessageWrapper_InnerMessage::ChatMsg(_) => (),
          _ => {
            self.inner_message = ServerMessageWrapper_InnerMessage::ChatMsg(::std::default::Default::default());
          },
        }
        if let ServerMessageWrapper_InnerMessage::ChatMsg(ref mut val) = self.inner_message {
          return ::pb_jelly::reflection::FieldMut::Value(val);
        }
        unreachable!()
      }
      "user_join_msg" => {
        match self.inner_message {
          ServerMessageWrapper_InnerMessage::UserJoinMsg(_) => (),
          _ => {
            self.inner_message = ServerMessageWrapper_InnerMessage::UserJoinMsg(::std::default::Default::default());
          },
        }
        if let ServerMessageWrapper_InnerMessage::UserJoinMsg(ref mut val) = self.inner_message {
          return ::pb_jelly::reflection::FieldMut::Value(val);
        }
        unreachable!()
      }
      "user_left_msg" => {
        match self.inner_message {
          ServerMessageWrapper_InnerMessage::UserLeftMsg(_) => (),
          _ => {
            self.inner_message = ServerMessageWrapper_InnerMessage::UserLeftMsg(::std::default::Default::default());
          },
        }
        if let ServerMessageWrapper_InnerMessage::UserLeftMsg(ref mut val) = self.inner_message {
          return ::pb_jelly::reflection::FieldMut::Value(val);
        }
        unreachable!()
      }
      _ => {
        panic!("unknown field name given")
      }
    }
  }
}

