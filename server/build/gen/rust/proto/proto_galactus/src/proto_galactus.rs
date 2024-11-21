// @generated, do not edit
/// The status of a task
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct TaskStatus(i32);
impl TaskStatus {
  pub const NOT_STARTED: TaskStatus = TaskStatus(0);
  pub const IN_PROGRESS: TaskStatus = TaskStatus(1);
  pub const COMPLETED: TaskStatus = TaskStatus(2);
  pub const FAILED: TaskStatus = TaskStatus(3);
  pub const KNOWN_VARIANTS: [TaskStatus; 4] = [TaskStatus::NOT_STARTED, TaskStatus::IN_PROGRESS, TaskStatus::COMPLETED, TaskStatus::FAILED];
  pub const fn value(self) -> i32 {
    self.0
  }
}
impl ::std::default::Default for TaskStatus {
  fn default() -> Self {
    TaskStatus::NOT_STARTED
  }
}
impl From<TaskStatus> for i32 {
  fn from(v: TaskStatus) -> i32 {
    v.0
  }
}
impl From<i32> for TaskStatus {
  fn from(v: i32) -> TaskStatus {
    TaskStatus(v)
  }
}
impl From<TaskStatus_Closed> for TaskStatus {
  fn from(v: TaskStatus_Closed) -> TaskStatus {
    TaskStatus(v as i32)
  }
}
impl ::pb_jelly::ProtoEnum for TaskStatus {
}
impl ::pb_jelly::OpenProtoEnum for TaskStatus {
  type Closed = TaskStatus_Closed;
  fn into_known(self) -> ::std::option::Option<TaskStatus_Closed> {
    match self {
      TaskStatus::NOT_STARTED => ::std::option::Option::Some(TaskStatus_Closed::NOT_STARTED),
      TaskStatus::IN_PROGRESS => ::std::option::Option::Some(TaskStatus_Closed::IN_PROGRESS),
      TaskStatus::COMPLETED => ::std::option::Option::Some(TaskStatus_Closed::COMPLETED),
      TaskStatus::FAILED => ::std::option::Option::Some(TaskStatus_Closed::FAILED),
      _ => None,
    }
  }
}
impl ::std::fmt::Debug for TaskStatus {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    match <Self as ::pb_jelly::OpenProtoEnum>::name(*self) {
      Some(s) => write!(f, "{}", s),
      None => write!(f, "Unknown({})", self.0),
    }
  }
}
/// The status of a task
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
#[repr(i32)]
pub enum TaskStatus_Closed {
  NOT_STARTED = 0,
  IN_PROGRESS = 1,
  COMPLETED = 2,
  FAILED = 3,
}
impl TaskStatus_Closed {
  pub const KNOWN_VARIANTS: [TaskStatus_Closed; 4] = [TaskStatus_Closed::NOT_STARTED, TaskStatus_Closed::IN_PROGRESS, TaskStatus_Closed::COMPLETED, TaskStatus_Closed::FAILED];
}
impl ::std::default::Default for TaskStatus_Closed {
  fn default() -> Self {
    TaskStatus_Closed::NOT_STARTED
  }
}
impl From<TaskStatus_Closed> for i32 {
  fn from(v: TaskStatus_Closed) -> i32 {
    match v {
      TaskStatus_Closed::NOT_STARTED => 0,
      TaskStatus_Closed::IN_PROGRESS => 1,
      TaskStatus_Closed::COMPLETED => 2,
      TaskStatus_Closed::FAILED => 3,
    }
  }
}
impl ::std::convert::TryFrom<i32> for TaskStatus_Closed {
  type Error = i32;
  fn try_from(v: i32) -> ::std::result::Result<Self, i32> {
    match v {
      0 => Ok(TaskStatus_Closed::NOT_STARTED),
      1 => Ok(TaskStatus_Closed::IN_PROGRESS),
      2 => Ok(TaskStatus_Closed::COMPLETED),
      3 => Ok(TaskStatus_Closed::FAILED),
      _ => Err(v),
    }
  }
}
impl ::pb_jelly::ProtoEnum for TaskStatus_Closed {
}
impl ::pb_jelly::ClosedProtoEnum for TaskStatus_Closed {
  fn name(self) -> &'static str {
    match self {
      TaskStatus_Closed::NOT_STARTED => "NOT_STARTED",
      TaskStatus_Closed::IN_PROGRESS => "IN_PROGRESS",
      TaskStatus_Closed::COMPLETED => "COMPLETED",
      TaskStatus_Closed::FAILED => "FAILED",
    }
  }
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Empty {
}
impl ::std::default::Default for Empty {
  fn default() -> Self {
    Empty {
    }
  }
}
::lazy_static::lazy_static! {
  pub static ref Empty_default: Empty = Empty::default();
}
impl ::pb_jelly::Message for Empty {
  fn descriptor(&self) -> ::std::option::Option<::pb_jelly::MessageDescriptor> {
    Some(::pb_jelly::MessageDescriptor {
      name: "Empty",
      full_name: "galactus.Empty",
      fields: &[
      ],
      oneofs: &[
      ],
    })
  }
  fn compute_size(&self) -> usize {
    0usize
  }
  fn serialize<W: ::pb_jelly::PbBufferWriter>(&self, w: &mut W) -> ::std::io::Result<()> {
    Ok(())
  }
  fn deserialize<B: ::pb_jelly::PbBufferReader>(&mut self, mut buf: &mut B) -> ::std::io::Result<()> {
    while let Some((field_number, typ)) = ::pb_jelly::wire_format::read(&mut buf)? {
      match field_number {
        _ => {
          ::pb_jelly::skip(typ, &mut buf)?;
        }
      }
    }
    Ok(())
  }
}
impl ::pb_jelly::Reflection for Empty {
  fn which_one_of(&self, oneof_name: &str) -> ::std::option::Option<&'static str> {
    match oneof_name {
      _ => {
        panic!("unknown oneof name given");
      }
    }
  }
  fn get_field_mut(&mut self, field_name: &str) -> ::pb_jelly::reflection::FieldMut<'_> {
    match field_name {
      _ => {
        panic!("unknown field name given")
      }
    }
  }
}

// Workers ___________________________________________________
// Workers are the entities that execute tasks.

/// Uniquely identifies a worker for the duration of its lifetime
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct WorkerID {
  pub worker_uuid: ::std::vec::Vec<u8>,
}
impl ::std::default::Default for WorkerID {
  fn default() -> Self {
    WorkerID {
      worker_uuid: ::std::default::Default::default(),
    }
  }
}
::lazy_static::lazy_static! {
  pub static ref WorkerID_default: WorkerID = WorkerID::default();
}
impl ::pb_jelly::Message for WorkerID {
  fn descriptor(&self) -> ::std::option::Option<::pb_jelly::MessageDescriptor> {
    Some(::pb_jelly::MessageDescriptor {
      name: "WorkerID",
      full_name: "galactus.WorkerID",
      fields: &[
        ::pb_jelly::FieldDescriptor {
          name: "worker_uuid",
          full_name: "galactus.WorkerID.worker_uuid",
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
    let mut size = 0usize;
    size += ::pb_jelly::helpers::compute_size_scalar::<::std::vec::Vec<u8>>(&self.worker_uuid, 1, ::pb_jelly::wire_format::Type::LengthDelimited);
    size
  }
  fn serialize<W: ::pb_jelly::PbBufferWriter>(&self, w: &mut W) -> ::std::io::Result<()> {
    ::pb_jelly::helpers::serialize_scalar::<W, ::std::vec::Vec<u8>>(w, &self.worker_uuid, 1, ::pb_jelly::wire_format::Type::LengthDelimited)?;
    Ok(())
  }
  fn deserialize<B: ::pb_jelly::PbBufferReader>(&mut self, mut buf: &mut B) -> ::std::io::Result<()> {
    while let Some((field_number, typ)) = ::pb_jelly::wire_format::read(&mut buf)? {
      match field_number {
        1 => {
          let val = ::pb_jelly::helpers::deserialize_length_delimited::<B, ::std::vec::Vec<u8>>(buf, typ, "WorkerID", 1)?;
          self.worker_uuid = val;
        }
        _ => {
          ::pb_jelly::skip(typ, &mut buf)?;
        }
      }
    }
    Ok(())
  }
}
impl ::pb_jelly::Reflection for WorkerID {
  fn which_one_of(&self, oneof_name: &str) -> ::std::option::Option<&'static str> {
    match oneof_name {
      _ => {
        panic!("unknown oneof name given");
      }
    }
  }
  fn get_field_mut(&mut self, field_name: &str) -> ::pb_jelly::reflection::FieldMut<'_> {
    match field_name {
      "worker_uuid" => {
        ::pb_jelly::reflection::FieldMut::Value(&mut self.worker_uuid)
      }
      _ => {
        panic!("unknown field name given")
      }
    }
  }
}

/// Metadata about a worker
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct WorkerMetadata {
  pub worker_id: ::std::option::Option<WorkerID>,
  pub supported_task_types: ::std::vec::Vec<::std::string::String>,
  pub max_slots: i32,
}
impl ::std::default::Default for WorkerMetadata {
  fn default() -> Self {
    WorkerMetadata {
      worker_id: ::std::default::Default::default(),
      supported_task_types: ::std::default::Default::default(),
      max_slots: ::std::default::Default::default(),
    }
  }
}
::lazy_static::lazy_static! {
  pub static ref WorkerMetadata_default: WorkerMetadata = WorkerMetadata::default();
}
impl ::pb_jelly::Message for WorkerMetadata {
  fn descriptor(&self) -> ::std::option::Option<::pb_jelly::MessageDescriptor> {
    Some(::pb_jelly::MessageDescriptor {
      name: "WorkerMetadata",
      full_name: "galactus.WorkerMetadata",
      fields: &[
        ::pb_jelly::FieldDescriptor {
          name: "worker_id",
          full_name: "galactus.WorkerMetadata.worker_id",
          index: 0,
          number: 1,
          typ: ::pb_jelly::wire_format::Type::LengthDelimited,
          label: ::pb_jelly::Label::Optional,
          oneof_index: None,
        },
        ::pb_jelly::FieldDescriptor {
          name: "supported_task_types",
          full_name: "galactus.WorkerMetadata.supported_task_types",
          index: 1,
          number: 2,
          typ: ::pb_jelly::wire_format::Type::LengthDelimited,
          label: ::pb_jelly::Label::Repeated,
          oneof_index: None,
        },
        ::pb_jelly::FieldDescriptor {
          name: "max_slots",
          full_name: "galactus.WorkerMetadata.max_slots",
          index: 2,
          number: 3,
          typ: ::pb_jelly::wire_format::Type::Varint,
          label: ::pb_jelly::Label::Optional,
          oneof_index: None,
        },
      ],
      oneofs: &[
      ],
    })
  }
  fn compute_size(&self) -> usize {
    let mut size = 0usize;
    if let Some(ref val) = self.worker_id {
      size += ::pb_jelly::helpers::compute_size_field::<WorkerID>(val, 1, ::pb_jelly::wire_format::Type::LengthDelimited);
    }
    for val in &self.supported_task_types {
      size += ::pb_jelly::helpers::compute_size_field::<::std::string::String>(val, 2, ::pb_jelly::wire_format::Type::LengthDelimited);
    }
    size += ::pb_jelly::helpers::compute_size_scalar::<i32>(&self.max_slots, 3, ::pb_jelly::wire_format::Type::Varint);
    size
  }
  fn serialize<W: ::pb_jelly::PbBufferWriter>(&self, w: &mut W) -> ::std::io::Result<()> {
    if let Some(ref val) = self.worker_id {
      ::pb_jelly::helpers::serialize_field::<W, WorkerID>(w, val, 1, ::pb_jelly::wire_format::Type::LengthDelimited)?;
    }
    for val in &self.supported_task_types {
      ::pb_jelly::helpers::serialize_field::<W, ::std::string::String>(w, val, 2, ::pb_jelly::wire_format::Type::LengthDelimited)?;
    }
    ::pb_jelly::helpers::serialize_scalar::<W, i32>(w, &self.max_slots, 3, ::pb_jelly::wire_format::Type::Varint)?;
    Ok(())
  }
  fn deserialize<B: ::pb_jelly::PbBufferReader>(&mut self, mut buf: &mut B) -> ::std::io::Result<()> {
    while let Some((field_number, typ)) = ::pb_jelly::wire_format::read(&mut buf)? {
      match field_number {
        1 => {
          let val = ::pb_jelly::helpers::deserialize_length_delimited::<B, WorkerID>(buf, typ, "WorkerMetadata", 1)?;
          self.worker_id = Some(val);
        }
        2 => {
          let val = ::pb_jelly::helpers::deserialize_length_delimited::<B, ::std::string::String>(buf, typ, "WorkerMetadata", 2)?;
          self.supported_task_types.push(val);
        }
        3 => {
          let val = ::pb_jelly::helpers::deserialize_known_length::<B, i32>(buf, typ, ::pb_jelly::wire_format::Type::Varint, "WorkerMetadata", 3)?;
          self.max_slots = val;
        }
        _ => {
          ::pb_jelly::skip(typ, &mut buf)?;
        }
      }
    }
    Ok(())
  }
}
impl ::pb_jelly::Reflection for WorkerMetadata {
  fn which_one_of(&self, oneof_name: &str) -> ::std::option::Option<&'static str> {
    match oneof_name {
      _ => {
        panic!("unknown oneof name given");
      }
    }
  }
  fn get_field_mut(&mut self, field_name: &str) -> ::pb_jelly::reflection::FieldMut<'_> {
    match field_name {
      "worker_id" => {
        ::pb_jelly::reflection::FieldMut::Value(self.worker_id.get_or_insert_with(::std::default::Default::default))
      }
      "supported_task_types" => {
        unimplemented!("Repeated fields are not currently supported.")
      }
      "max_slots" => {
        ::pb_jelly::reflection::FieldMut::Value(&mut self.max_slots)
      }
      _ => {
        panic!("unknown field name given")
      }
    }
  }
}

/// Status of a worker
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct WorkerStatus {
  /// The worker's ID
  pub worker_id: ::std::option::Option<WorkerID>,
  /// The tasks the worker is currently running
  pub tasks: ::std::vec::Vec<TaskID>,
  /// The worker's current capacity
  pub available_slots: i32,
  /// Whether the worker is accepting new tasks
  pub accepting_tasks: bool,
}
impl ::std::default::Default for WorkerStatus {
  fn default() -> Self {
    WorkerStatus {
      worker_id: ::std::default::Default::default(),
      tasks: ::std::default::Default::default(),
      available_slots: ::std::default::Default::default(),
      accepting_tasks: ::std::default::Default::default(),
    }
  }
}
::lazy_static::lazy_static! {
  pub static ref WorkerStatus_default: WorkerStatus = WorkerStatus::default();
}
impl ::pb_jelly::Message for WorkerStatus {
  fn descriptor(&self) -> ::std::option::Option<::pb_jelly::MessageDescriptor> {
    Some(::pb_jelly::MessageDescriptor {
      name: "WorkerStatus",
      full_name: "galactus.WorkerStatus",
      fields: &[
        ::pb_jelly::FieldDescriptor {
          name: "worker_id",
          full_name: "galactus.WorkerStatus.worker_id",
          index: 0,
          number: 1,
          typ: ::pb_jelly::wire_format::Type::LengthDelimited,
          label: ::pb_jelly::Label::Optional,
          oneof_index: None,
        },
        ::pb_jelly::FieldDescriptor {
          name: "tasks",
          full_name: "galactus.WorkerStatus.tasks",
          index: 1,
          number: 2,
          typ: ::pb_jelly::wire_format::Type::LengthDelimited,
          label: ::pb_jelly::Label::Repeated,
          oneof_index: None,
        },
        ::pb_jelly::FieldDescriptor {
          name: "available_slots",
          full_name: "galactus.WorkerStatus.available_slots",
          index: 2,
          number: 3,
          typ: ::pb_jelly::wire_format::Type::Varint,
          label: ::pb_jelly::Label::Optional,
          oneof_index: None,
        },
        ::pb_jelly::FieldDescriptor {
          name: "accepting_tasks",
          full_name: "galactus.WorkerStatus.accepting_tasks",
          index: 3,
          number: 4,
          typ: ::pb_jelly::wire_format::Type::Varint,
          label: ::pb_jelly::Label::Optional,
          oneof_index: None,
        },
      ],
      oneofs: &[
      ],
    })
  }
  fn compute_size(&self) -> usize {
    let mut size = 0usize;
    if let Some(ref val) = self.worker_id {
      size += ::pb_jelly::helpers::compute_size_field::<WorkerID>(val, 1, ::pb_jelly::wire_format::Type::LengthDelimited);
    }
    for val in &self.tasks {
      size += ::pb_jelly::helpers::compute_size_field::<TaskID>(val, 2, ::pb_jelly::wire_format::Type::LengthDelimited);
    }
    size += ::pb_jelly::helpers::compute_size_scalar::<i32>(&self.available_slots, 3, ::pb_jelly::wire_format::Type::Varint);
    size += ::pb_jelly::helpers::compute_size_scalar::<bool>(&self.accepting_tasks, 4, ::pb_jelly::wire_format::Type::Varint);
    size
  }
  fn serialize<W: ::pb_jelly::PbBufferWriter>(&self, w: &mut W) -> ::std::io::Result<()> {
    if let Some(ref val) = self.worker_id {
      ::pb_jelly::helpers::serialize_field::<W, WorkerID>(w, val, 1, ::pb_jelly::wire_format::Type::LengthDelimited)?;
    }
    for val in &self.tasks {
      ::pb_jelly::helpers::serialize_field::<W, TaskID>(w, val, 2, ::pb_jelly::wire_format::Type::LengthDelimited)?;
    }
    ::pb_jelly::helpers::serialize_scalar::<W, i32>(w, &self.available_slots, 3, ::pb_jelly::wire_format::Type::Varint)?;
    ::pb_jelly::helpers::serialize_scalar::<W, bool>(w, &self.accepting_tasks, 4, ::pb_jelly::wire_format::Type::Varint)?;
    Ok(())
  }
  fn deserialize<B: ::pb_jelly::PbBufferReader>(&mut self, mut buf: &mut B) -> ::std::io::Result<()> {
    while let Some((field_number, typ)) = ::pb_jelly::wire_format::read(&mut buf)? {
      match field_number {
        1 => {
          let val = ::pb_jelly::helpers::deserialize_length_delimited::<B, WorkerID>(buf, typ, "WorkerStatus", 1)?;
          self.worker_id = Some(val);
        }
        2 => {
          let val = ::pb_jelly::helpers::deserialize_length_delimited::<B, TaskID>(buf, typ, "WorkerStatus", 2)?;
          self.tasks.push(val);
        }
        3 => {
          let val = ::pb_jelly::helpers::deserialize_known_length::<B, i32>(buf, typ, ::pb_jelly::wire_format::Type::Varint, "WorkerStatus", 3)?;
          self.available_slots = val;
        }
        4 => {
          let val = ::pb_jelly::helpers::deserialize_known_length::<B, bool>(buf, typ, ::pb_jelly::wire_format::Type::Varint, "WorkerStatus", 4)?;
          self.accepting_tasks = val;
        }
        _ => {
          ::pb_jelly::skip(typ, &mut buf)?;
        }
      }
    }
    Ok(())
  }
}
impl ::pb_jelly::Reflection for WorkerStatus {
  fn which_one_of(&self, oneof_name: &str) -> ::std::option::Option<&'static str> {
    match oneof_name {
      _ => {
        panic!("unknown oneof name given");
      }
    }
  }
  fn get_field_mut(&mut self, field_name: &str) -> ::pb_jelly::reflection::FieldMut<'_> {
    match field_name {
      "worker_id" => {
        ::pb_jelly::reflection::FieldMut::Value(self.worker_id.get_or_insert_with(::std::default::Default::default))
      }
      "tasks" => {
        unimplemented!("Repeated fields are not currently supported.")
      }
      "available_slots" => {
        ::pb_jelly::reflection::FieldMut::Value(&mut self.available_slots)
      }
      "accepting_tasks" => {
        ::pb_jelly::reflection::FieldMut::Value(&mut self.accepting_tasks)
      }
      _ => {
        panic!("unknown field name given")
      }
    }
  }
}

/// Message sent from workers to Galactus when they register
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct WorkerRegistration {
  /// The worker's metadata
  pub worker_metadata: ::std::option::Option<WorkerMetadata>,
  /// The worker's status
  pub worker_status: ::std::option::Option<WorkerStatus>,
}
impl ::std::default::Default for WorkerRegistration {
  fn default() -> Self {
    WorkerRegistration {
      worker_metadata: ::std::default::Default::default(),
      worker_status: ::std::default::Default::default(),
    }
  }
}
::lazy_static::lazy_static! {
  pub static ref WorkerRegistration_default: WorkerRegistration = WorkerRegistration::default();
}
impl ::pb_jelly::Message for WorkerRegistration {
  fn descriptor(&self) -> ::std::option::Option<::pb_jelly::MessageDescriptor> {
    Some(::pb_jelly::MessageDescriptor {
      name: "WorkerRegistration",
      full_name: "galactus.WorkerRegistration",
      fields: &[
        ::pb_jelly::FieldDescriptor {
          name: "worker_metadata",
          full_name: "galactus.WorkerRegistration.worker_metadata",
          index: 0,
          number: 1,
          typ: ::pb_jelly::wire_format::Type::LengthDelimited,
          label: ::pb_jelly::Label::Optional,
          oneof_index: None,
        },
        ::pb_jelly::FieldDescriptor {
          name: "worker_status",
          full_name: "galactus.WorkerRegistration.worker_status",
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
    let mut size = 0usize;
    if let Some(ref val) = self.worker_metadata {
      size += ::pb_jelly::helpers::compute_size_field::<WorkerMetadata>(val, 1, ::pb_jelly::wire_format::Type::LengthDelimited);
    }
    if let Some(ref val) = self.worker_status {
      size += ::pb_jelly::helpers::compute_size_field::<WorkerStatus>(val, 2, ::pb_jelly::wire_format::Type::LengthDelimited);
    }
    size
  }
  fn serialize<W: ::pb_jelly::PbBufferWriter>(&self, w: &mut W) -> ::std::io::Result<()> {
    if let Some(ref val) = self.worker_metadata {
      ::pb_jelly::helpers::serialize_field::<W, WorkerMetadata>(w, val, 1, ::pb_jelly::wire_format::Type::LengthDelimited)?;
    }
    if let Some(ref val) = self.worker_status {
      ::pb_jelly::helpers::serialize_field::<W, WorkerStatus>(w, val, 2, ::pb_jelly::wire_format::Type::LengthDelimited)?;
    }
    Ok(())
  }
  fn deserialize<B: ::pb_jelly::PbBufferReader>(&mut self, mut buf: &mut B) -> ::std::io::Result<()> {
    while let Some((field_number, typ)) = ::pb_jelly::wire_format::read(&mut buf)? {
      match field_number {
        1 => {
          let val = ::pb_jelly::helpers::deserialize_length_delimited::<B, WorkerMetadata>(buf, typ, "WorkerRegistration", 1)?;
          self.worker_metadata = Some(val);
        }
        2 => {
          let val = ::pb_jelly::helpers::deserialize_length_delimited::<B, WorkerStatus>(buf, typ, "WorkerRegistration", 2)?;
          self.worker_status = Some(val);
        }
        _ => {
          ::pb_jelly::skip(typ, &mut buf)?;
        }
      }
    }
    Ok(())
  }
}
impl ::pb_jelly::Reflection for WorkerRegistration {
  fn which_one_of(&self, oneof_name: &str) -> ::std::option::Option<&'static str> {
    match oneof_name {
      _ => {
        panic!("unknown oneof name given");
      }
    }
  }
  fn get_field_mut(&mut self, field_name: &str) -> ::pb_jelly::reflection::FieldMut<'_> {
    match field_name {
      "worker_metadata" => {
        ::pb_jelly::reflection::FieldMut::Value(self.worker_metadata.get_or_insert_with(::std::default::Default::default))
      }
      "worker_status" => {
        ::pb_jelly::reflection::FieldMut::Value(self.worker_status.get_or_insert_with(::std::default::Default::default))
      }
      _ => {
        panic!("unknown field name given")
      }
    }
  }
}

// Tasks ___________________________________________________
// Task data is passed between Galactus and the workers to
// coordinate task execution and return results.

/// A unique identifier for a task
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct TaskID {
  pub task_uuid: ::std::vec::Vec<u8>,
}
impl ::std::default::Default for TaskID {
  fn default() -> Self {
    TaskID {
      task_uuid: ::std::default::Default::default(),
    }
  }
}
::lazy_static::lazy_static! {
  pub static ref TaskID_default: TaskID = TaskID::default();
}
impl ::pb_jelly::Message for TaskID {
  fn descriptor(&self) -> ::std::option::Option<::pb_jelly::MessageDescriptor> {
    Some(::pb_jelly::MessageDescriptor {
      name: "TaskID",
      full_name: "galactus.TaskID",
      fields: &[
        ::pb_jelly::FieldDescriptor {
          name: "task_uuid",
          full_name: "galactus.TaskID.task_uuid",
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
    let mut size = 0usize;
    size += ::pb_jelly::helpers::compute_size_scalar::<::std::vec::Vec<u8>>(&self.task_uuid, 1, ::pb_jelly::wire_format::Type::LengthDelimited);
    size
  }
  fn serialize<W: ::pb_jelly::PbBufferWriter>(&self, w: &mut W) -> ::std::io::Result<()> {
    ::pb_jelly::helpers::serialize_scalar::<W, ::std::vec::Vec<u8>>(w, &self.task_uuid, 1, ::pb_jelly::wire_format::Type::LengthDelimited)?;
    Ok(())
  }
  fn deserialize<B: ::pb_jelly::PbBufferReader>(&mut self, mut buf: &mut B) -> ::std::io::Result<()> {
    while let Some((field_number, typ)) = ::pb_jelly::wire_format::read(&mut buf)? {
      match field_number {
        1 => {
          let val = ::pb_jelly::helpers::deserialize_length_delimited::<B, ::std::vec::Vec<u8>>(buf, typ, "TaskID", 1)?;
          self.task_uuid = val;
        }
        _ => {
          ::pb_jelly::skip(typ, &mut buf)?;
        }
      }
    }
    Ok(())
  }
}
impl ::pb_jelly::Reflection for TaskID {
  fn which_one_of(&self, oneof_name: &str) -> ::std::option::Option<&'static str> {
    match oneof_name {
      _ => {
        panic!("unknown oneof name given");
      }
    }
  }
  fn get_field_mut(&mut self, field_name: &str) -> ::pb_jelly::reflection::FieldMut<'_> {
    match field_name {
      "task_uuid" => {
        ::pb_jelly::reflection::FieldMut::Value(&mut self.task_uuid)
      }
      _ => {
        panic!("unknown field name given")
      }
    }
  }
}

// Task Execution -----

/// Data passed into the task function
#[derive(Clone, Debug, PartialEq)]
pub struct TaskExecutionData {
  /// The command to execute
  pub task_command: ::std::string::String,
  /// Arbitrary data passed into the task function
  pub task_data: ::std::option::Option<::proto_google::protobuf::r#struct::Struct>,
}
impl ::std::default::Default for TaskExecutionData {
  fn default() -> Self {
    TaskExecutionData {
      task_command: ::std::default::Default::default(),
      task_data: ::std::default::Default::default(),
    }
  }
}
::lazy_static::lazy_static! {
  pub static ref TaskExecutionData_default: TaskExecutionData = TaskExecutionData::default();
}
impl ::pb_jelly::Message for TaskExecutionData {
  fn descriptor(&self) -> ::std::option::Option<::pb_jelly::MessageDescriptor> {
    Some(::pb_jelly::MessageDescriptor {
      name: "TaskExecutionData",
      full_name: "galactus.TaskExecutionData",
      fields: &[
        ::pb_jelly::FieldDescriptor {
          name: "task_command",
          full_name: "galactus.TaskExecutionData.task_command",
          index: 0,
          number: 1,
          typ: ::pb_jelly::wire_format::Type::LengthDelimited,
          label: ::pb_jelly::Label::Optional,
          oneof_index: None,
        },
        ::pb_jelly::FieldDescriptor {
          name: "task_data",
          full_name: "galactus.TaskExecutionData.task_data",
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
    let mut size = 0usize;
    size += ::pb_jelly::helpers::compute_size_scalar::<::std::string::String>(&self.task_command, 1, ::pb_jelly::wire_format::Type::LengthDelimited);
    if let Some(ref val) = self.task_data {
      size += ::pb_jelly::helpers::compute_size_field::<::proto_google::protobuf::r#struct::Struct>(val, 2, ::pb_jelly::wire_format::Type::LengthDelimited);
    }
    size
  }
  fn serialize<W: ::pb_jelly::PbBufferWriter>(&self, w: &mut W) -> ::std::io::Result<()> {
    ::pb_jelly::helpers::serialize_scalar::<W, ::std::string::String>(w, &self.task_command, 1, ::pb_jelly::wire_format::Type::LengthDelimited)?;
    if let Some(ref val) = self.task_data {
      ::pb_jelly::helpers::serialize_field::<W, ::proto_google::protobuf::r#struct::Struct>(w, val, 2, ::pb_jelly::wire_format::Type::LengthDelimited)?;
    }
    Ok(())
  }
  fn deserialize<B: ::pb_jelly::PbBufferReader>(&mut self, mut buf: &mut B) -> ::std::io::Result<()> {
    while let Some((field_number, typ)) = ::pb_jelly::wire_format::read(&mut buf)? {
      match field_number {
        1 => {
          let val = ::pb_jelly::helpers::deserialize_length_delimited::<B, ::std::string::String>(buf, typ, "TaskExecutionData", 1)?;
          self.task_command = val;
        }
        2 => {
          let val = ::pb_jelly::helpers::deserialize_length_delimited::<B, ::proto_google::protobuf::r#struct::Struct>(buf, typ, "TaskExecutionData", 2)?;
          self.task_data = Some(val);
        }
        _ => {
          ::pb_jelly::skip(typ, &mut buf)?;
        }
      }
    }
    Ok(())
  }
}
impl ::pb_jelly::Reflection for TaskExecutionData {
  fn which_one_of(&self, oneof_name: &str) -> ::std::option::Option<&'static str> {
    match oneof_name {
      _ => {
        panic!("unknown oneof name given");
      }
    }
  }
  fn get_field_mut(&mut self, field_name: &str) -> ::pb_jelly::reflection::FieldMut<'_> {
    match field_name {
      "task_command" => {
        ::pb_jelly::reflection::FieldMut::Value(&mut self.task_command)
      }
      "task_data" => {
        ::pb_jelly::reflection::FieldMut::Value(self.task_data.get_or_insert_with(::std::default::Default::default))
      }
      _ => {
        panic!("unknown field name given")
      }
    }
  }
}

/// A task assigned to a worker
#[derive(Clone, Debug, PartialEq)]
pub struct TaskAssignment {
  /// The task's ID
  pub task_id: ::std::option::Option<TaskID>,
  /// The task's priority. If higher than currently running tasks, the worker will
  /// interrupt the lower priority tasks to run this one.
  pub priority: i32,
  /// Data passed into the task function
  pub task_data: ::std::option::Option<TaskExecutionData>,
}
impl ::std::default::Default for TaskAssignment {
  fn default() -> Self {
    TaskAssignment {
      task_id: ::std::default::Default::default(),
      priority: ::std::default::Default::default(),
      task_data: ::std::default::Default::default(),
    }
  }
}
::lazy_static::lazy_static! {
  pub static ref TaskAssignment_default: TaskAssignment = TaskAssignment::default();
}
impl ::pb_jelly::Message for TaskAssignment {
  fn descriptor(&self) -> ::std::option::Option<::pb_jelly::MessageDescriptor> {
    Some(::pb_jelly::MessageDescriptor {
      name: "TaskAssignment",
      full_name: "galactus.TaskAssignment",
      fields: &[
        ::pb_jelly::FieldDescriptor {
          name: "task_id",
          full_name: "galactus.TaskAssignment.task_id",
          index: 0,
          number: 1,
          typ: ::pb_jelly::wire_format::Type::LengthDelimited,
          label: ::pb_jelly::Label::Optional,
          oneof_index: None,
        },
        ::pb_jelly::FieldDescriptor {
          name: "priority",
          full_name: "galactus.TaskAssignment.priority",
          index: 1,
          number: 2,
          typ: ::pb_jelly::wire_format::Type::Varint,
          label: ::pb_jelly::Label::Optional,
          oneof_index: None,
        },
        ::pb_jelly::FieldDescriptor {
          name: "task_data",
          full_name: "galactus.TaskAssignment.task_data",
          index: 2,
          number: 3,
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
    let mut size = 0usize;
    if let Some(ref val) = self.task_id {
      size += ::pb_jelly::helpers::compute_size_field::<TaskID>(val, 1, ::pb_jelly::wire_format::Type::LengthDelimited);
    }
    size += ::pb_jelly::helpers::compute_size_scalar::<i32>(&self.priority, 2, ::pb_jelly::wire_format::Type::Varint);
    if let Some(ref val) = self.task_data {
      size += ::pb_jelly::helpers::compute_size_field::<TaskExecutionData>(val, 3, ::pb_jelly::wire_format::Type::LengthDelimited);
    }
    size
  }
  fn serialize<W: ::pb_jelly::PbBufferWriter>(&self, w: &mut W) -> ::std::io::Result<()> {
    if let Some(ref val) = self.task_id {
      ::pb_jelly::helpers::serialize_field::<W, TaskID>(w, val, 1, ::pb_jelly::wire_format::Type::LengthDelimited)?;
    }
    ::pb_jelly::helpers::serialize_scalar::<W, i32>(w, &self.priority, 2, ::pb_jelly::wire_format::Type::Varint)?;
    if let Some(ref val) = self.task_data {
      ::pb_jelly::helpers::serialize_field::<W, TaskExecutionData>(w, val, 3, ::pb_jelly::wire_format::Type::LengthDelimited)?;
    }
    Ok(())
  }
  fn deserialize<B: ::pb_jelly::PbBufferReader>(&mut self, mut buf: &mut B) -> ::std::io::Result<()> {
    while let Some((field_number, typ)) = ::pb_jelly::wire_format::read(&mut buf)? {
      match field_number {
        1 => {
          let val = ::pb_jelly::helpers::deserialize_length_delimited::<B, TaskID>(buf, typ, "TaskAssignment", 1)?;
          self.task_id = Some(val);
        }
        2 => {
          let val = ::pb_jelly::helpers::deserialize_known_length::<B, i32>(buf, typ, ::pb_jelly::wire_format::Type::Varint, "TaskAssignment", 2)?;
          self.priority = val;
        }
        3 => {
          let val = ::pb_jelly::helpers::deserialize_length_delimited::<B, TaskExecutionData>(buf, typ, "TaskAssignment", 3)?;
          self.task_data = Some(val);
        }
        _ => {
          ::pb_jelly::skip(typ, &mut buf)?;
        }
      }
    }
    Ok(())
  }
}
impl ::pb_jelly::Reflection for TaskAssignment {
  fn which_one_of(&self, oneof_name: &str) -> ::std::option::Option<&'static str> {
    match oneof_name {
      _ => {
        panic!("unknown oneof name given");
      }
    }
  }
  fn get_field_mut(&mut self, field_name: &str) -> ::pb_jelly::reflection::FieldMut<'_> {
    match field_name {
      "task_id" => {
        ::pb_jelly::reflection::FieldMut::Value(self.task_id.get_or_insert_with(::std::default::Default::default))
      }
      "priority" => {
        ::pb_jelly::reflection::FieldMut::Value(&mut self.priority)
      }
      "task_data" => {
        ::pb_jelly::reflection::FieldMut::Value(self.task_data.get_or_insert_with(::std::default::Default::default))
      }
      _ => {
        panic!("unknown field name given")
      }
    }
  }
}

// Task Results -----

/// Data returned from the task function
#[derive(Clone, Debug, PartialEq)]
pub struct TaskResultData {
  /// Arbitrary data returned from the task function
  pub task_data: ::std::option::Option<::proto_google::protobuf::r#struct::Struct>,
}
impl ::std::default::Default for TaskResultData {
  fn default() -> Self {
    TaskResultData {
      task_data: ::std::default::Default::default(),
    }
  }
}
::lazy_static::lazy_static! {
  pub static ref TaskResultData_default: TaskResultData = TaskResultData::default();
}
impl ::pb_jelly::Message for TaskResultData {
  fn descriptor(&self) -> ::std::option::Option<::pb_jelly::MessageDescriptor> {
    Some(::pb_jelly::MessageDescriptor {
      name: "TaskResultData",
      full_name: "galactus.TaskResultData",
      fields: &[
        ::pb_jelly::FieldDescriptor {
          name: "task_data",
          full_name: "galactus.TaskResultData.task_data",
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
    let mut size = 0usize;
    if let Some(ref val) = self.task_data {
      size += ::pb_jelly::helpers::compute_size_field::<::proto_google::protobuf::r#struct::Struct>(val, 1, ::pb_jelly::wire_format::Type::LengthDelimited);
    }
    size
  }
  fn serialize<W: ::pb_jelly::PbBufferWriter>(&self, w: &mut W) -> ::std::io::Result<()> {
    if let Some(ref val) = self.task_data {
      ::pb_jelly::helpers::serialize_field::<W, ::proto_google::protobuf::r#struct::Struct>(w, val, 1, ::pb_jelly::wire_format::Type::LengthDelimited)?;
    }
    Ok(())
  }
  fn deserialize<B: ::pb_jelly::PbBufferReader>(&mut self, mut buf: &mut B) -> ::std::io::Result<()> {
    while let Some((field_number, typ)) = ::pb_jelly::wire_format::read(&mut buf)? {
      match field_number {
        1 => {
          let val = ::pb_jelly::helpers::deserialize_length_delimited::<B, ::proto_google::protobuf::r#struct::Struct>(buf, typ, "TaskResultData", 1)?;
          self.task_data = Some(val);
        }
        _ => {
          ::pb_jelly::skip(typ, &mut buf)?;
        }
      }
    }
    Ok(())
  }
}
impl ::pb_jelly::Reflection for TaskResultData {
  fn which_one_of(&self, oneof_name: &str) -> ::std::option::Option<&'static str> {
    match oneof_name {
      _ => {
        panic!("unknown oneof name given");
      }
    }
  }
  fn get_field_mut(&mut self, field_name: &str) -> ::pb_jelly::reflection::FieldMut<'_> {
    match field_name {
      "task_data" => {
        ::pb_jelly::reflection::FieldMut::Value(self.task_data.get_or_insert_with(::std::default::Default::default))
      }
      _ => {
        panic!("unknown field name given")
      }
    }
  }
}

/// Data returned from the task function if the task failed
#[derive(Clone, Debug, PartialEq)]
pub struct TaskErrorData {
  /// The error can be arbitrary data. Ideally we should have ways of
  /// serializing the error into some format. TODO
  pub task_error: ::std::option::Option<::proto_google::protobuf::r#struct::Struct>,
}
impl ::std::default::Default for TaskErrorData {
  fn default() -> Self {
    TaskErrorData {
      task_error: ::std::default::Default::default(),
    }
  }
}
::lazy_static::lazy_static! {
  pub static ref TaskErrorData_default: TaskErrorData = TaskErrorData::default();
}
impl ::pb_jelly::Message for TaskErrorData {
  fn descriptor(&self) -> ::std::option::Option<::pb_jelly::MessageDescriptor> {
    Some(::pb_jelly::MessageDescriptor {
      name: "TaskErrorData",
      full_name: "galactus.TaskErrorData",
      fields: &[
        ::pb_jelly::FieldDescriptor {
          name: "task_error",
          full_name: "galactus.TaskErrorData.task_error",
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
    let mut size = 0usize;
    if let Some(ref val) = self.task_error {
      size += ::pb_jelly::helpers::compute_size_field::<::proto_google::protobuf::r#struct::Struct>(val, 1, ::pb_jelly::wire_format::Type::LengthDelimited);
    }
    size
  }
  fn serialize<W: ::pb_jelly::PbBufferWriter>(&self, w: &mut W) -> ::std::io::Result<()> {
    if let Some(ref val) = self.task_error {
      ::pb_jelly::helpers::serialize_field::<W, ::proto_google::protobuf::r#struct::Struct>(w, val, 1, ::pb_jelly::wire_format::Type::LengthDelimited)?;
    }
    Ok(())
  }
  fn deserialize<B: ::pb_jelly::PbBufferReader>(&mut self, mut buf: &mut B) -> ::std::io::Result<()> {
    while let Some((field_number, typ)) = ::pb_jelly::wire_format::read(&mut buf)? {
      match field_number {
        1 => {
          let val = ::pb_jelly::helpers::deserialize_length_delimited::<B, ::proto_google::protobuf::r#struct::Struct>(buf, typ, "TaskErrorData", 1)?;
          self.task_error = Some(val);
        }
        _ => {
          ::pb_jelly::skip(typ, &mut buf)?;
        }
      }
    }
    Ok(())
  }
}
impl ::pb_jelly::Reflection for TaskErrorData {
  fn which_one_of(&self, oneof_name: &str) -> ::std::option::Option<&'static str> {
    match oneof_name {
      _ => {
        panic!("unknown oneof name given");
      }
    }
  }
  fn get_field_mut(&mut self, field_name: &str) -> ::pb_jelly::reflection::FieldMut<'_> {
    match field_name {
      "task_error" => {
        ::pb_jelly::reflection::FieldMut::Value(self.task_error.get_or_insert_with(::std::default::Default::default))
      }
      _ => {
        panic!("unknown field name given")
      }
    }
  }
}

/// Result of a finalized task, including its status and result or error
#[derive(Clone, Debug, PartialEq)]
pub struct TaskUpdate {
  /// The task's ID
  pub task_id: ::std::option::Option<TaskID>,
  /// The task's status
  pub status: TaskStatus,
  /// The worker's status
  pub worker_status: ::std::option::Option<WorkerStatus>,
  pub result: ::std::option::Option<TaskUpdate_Result>,
}
#[derive(Clone, Debug, PartialEq)]
pub enum TaskUpdate_Result {
  TaskResult(TaskResultData),
  TaskError(TaskErrorData),
  TaskCancelled(Empty),
}
impl ::std::default::Default for TaskUpdate {
  fn default() -> Self {
    TaskUpdate {
      task_id: ::std::default::Default::default(),
      status: ::std::default::Default::default(),
      worker_status: ::std::default::Default::default(),
      result: None,
    }
  }
}
::lazy_static::lazy_static! {
  pub static ref TaskUpdate_default: TaskUpdate = TaskUpdate::default();
}
impl ::pb_jelly::Message for TaskUpdate {
  fn descriptor(&self) -> ::std::option::Option<::pb_jelly::MessageDescriptor> {
    Some(::pb_jelly::MessageDescriptor {
      name: "TaskUpdate",
      full_name: "galactus.TaskUpdate",
      fields: &[
        ::pb_jelly::FieldDescriptor {
          name: "task_id",
          full_name: "galactus.TaskUpdate.task_id",
          index: 0,
          number: 1,
          typ: ::pb_jelly::wire_format::Type::LengthDelimited,
          label: ::pb_jelly::Label::Optional,
          oneof_index: None,
        },
        ::pb_jelly::FieldDescriptor {
          name: "status",
          full_name: "galactus.TaskUpdate.status",
          index: 1,
          number: 2,
          typ: ::pb_jelly::wire_format::Type::Varint,
          label: ::pb_jelly::Label::Optional,
          oneof_index: None,
        },
        ::pb_jelly::FieldDescriptor {
          name: "worker_status",
          full_name: "galactus.TaskUpdate.worker_status",
          index: 2,
          number: 3,
          typ: ::pb_jelly::wire_format::Type::LengthDelimited,
          label: ::pb_jelly::Label::Optional,
          oneof_index: None,
        },
        ::pb_jelly::FieldDescriptor {
          name: "task_result",
          full_name: "galactus.TaskUpdate.task_result",
          index: 3,
          number: 4,
          typ: ::pb_jelly::wire_format::Type::LengthDelimited,
          label: ::pb_jelly::Label::Optional,
          oneof_index: Some(0),
        },
        ::pb_jelly::FieldDescriptor {
          name: "task_error",
          full_name: "galactus.TaskUpdate.task_error",
          index: 4,
          number: 5,
          typ: ::pb_jelly::wire_format::Type::LengthDelimited,
          label: ::pb_jelly::Label::Optional,
          oneof_index: Some(0),
        },
        ::pb_jelly::FieldDescriptor {
          name: "task_cancelled",
          full_name: "galactus.TaskUpdate.task_cancelled",
          index: 5,
          number: 6,
          typ: ::pb_jelly::wire_format::Type::LengthDelimited,
          label: ::pb_jelly::Label::Optional,
          oneof_index: Some(0),
        },
      ],
      oneofs: &[
        ::pb_jelly::OneofDescriptor {
          name: "result",
        },
      ],
    })
  }
  fn compute_size(&self) -> usize {
    let mut size = 0usize;
    if let Some(ref val) = self.task_id {
      size += ::pb_jelly::helpers::compute_size_field::<TaskID>(val, 1, ::pb_jelly::wire_format::Type::LengthDelimited);
    }
    size += ::pb_jelly::helpers::compute_size_scalar::<TaskStatus>(&self.status, 2, ::pb_jelly::wire_format::Type::Varint);
    if let Some(ref val) = self.worker_status {
      size += ::pb_jelly::helpers::compute_size_field::<WorkerStatus>(val, 3, ::pb_jelly::wire_format::Type::LengthDelimited);
    }
    if let Some(TaskUpdate_Result::TaskResult(ref val)) = self.result {
      size += ::pb_jelly::helpers::compute_size_field::<TaskResultData>(val, 4, ::pb_jelly::wire_format::Type::LengthDelimited);
    }
    if let Some(TaskUpdate_Result::TaskError(ref val)) = self.result {
      size += ::pb_jelly::helpers::compute_size_field::<TaskErrorData>(val, 5, ::pb_jelly::wire_format::Type::LengthDelimited);
    }
    if let Some(TaskUpdate_Result::TaskCancelled(ref val)) = self.result {
      size += ::pb_jelly::helpers::compute_size_field::<Empty>(val, 6, ::pb_jelly::wire_format::Type::LengthDelimited);
    }
    size
  }
  fn serialize<W: ::pb_jelly::PbBufferWriter>(&self, w: &mut W) -> ::std::io::Result<()> {
    if let Some(ref val) = self.task_id {
      ::pb_jelly::helpers::serialize_field::<W, TaskID>(w, val, 1, ::pb_jelly::wire_format::Type::LengthDelimited)?;
    }
    ::pb_jelly::helpers::serialize_scalar::<W, TaskStatus>(w, &self.status, 2, ::pb_jelly::wire_format::Type::Varint)?;
    if let Some(ref val) = self.worker_status {
      ::pb_jelly::helpers::serialize_field::<W, WorkerStatus>(w, val, 3, ::pb_jelly::wire_format::Type::LengthDelimited)?;
    }
    if let Some(TaskUpdate_Result::TaskResult(ref val)) = self.result {
      ::pb_jelly::helpers::serialize_field::<W, TaskResultData>(w, val, 4, ::pb_jelly::wire_format::Type::LengthDelimited)?;
    }
    if let Some(TaskUpdate_Result::TaskError(ref val)) = self.result {
      ::pb_jelly::helpers::serialize_field::<W, TaskErrorData>(w, val, 5, ::pb_jelly::wire_format::Type::LengthDelimited)?;
    }
    if let Some(TaskUpdate_Result::TaskCancelled(ref val)) = self.result {
      ::pb_jelly::helpers::serialize_field::<W, Empty>(w, val, 6, ::pb_jelly::wire_format::Type::LengthDelimited)?;
    }
    Ok(())
  }
  fn deserialize<B: ::pb_jelly::PbBufferReader>(&mut self, mut buf: &mut B) -> ::std::io::Result<()> {
    while let Some((field_number, typ)) = ::pb_jelly::wire_format::read(&mut buf)? {
      match field_number {
        1 => {
          let val = ::pb_jelly::helpers::deserialize_length_delimited::<B, TaskID>(buf, typ, "TaskUpdate", 1)?;
          self.task_id = Some(val);
        }
        2 => {
          let val = ::pb_jelly::helpers::deserialize_known_length::<B, TaskStatus>(buf, typ, ::pb_jelly::wire_format::Type::Varint, "TaskUpdate", 2)?;
          self.status = val;
        }
        3 => {
          let val = ::pb_jelly::helpers::deserialize_length_delimited::<B, WorkerStatus>(buf, typ, "TaskUpdate", 3)?;
          self.worker_status = Some(val);
        }
        4 => {
          let val = ::pb_jelly::helpers::deserialize_length_delimited::<B, TaskResultData>(buf, typ, "TaskUpdate", 4)?;
          self.result = Some(TaskUpdate_Result::TaskResult(val));
        }
        5 => {
          let val = ::pb_jelly::helpers::deserialize_length_delimited::<B, TaskErrorData>(buf, typ, "TaskUpdate", 5)?;
          self.result = Some(TaskUpdate_Result::TaskError(val));
        }
        6 => {
          let val = ::pb_jelly::helpers::deserialize_length_delimited::<B, Empty>(buf, typ, "TaskUpdate", 6)?;
          self.result = Some(TaskUpdate_Result::TaskCancelled(val));
        }
        _ => {
          ::pb_jelly::skip(typ, &mut buf)?;
        }
      }
    }
    Ok(())
  }
}
impl ::pb_jelly::Reflection for TaskUpdate {
  fn which_one_of(&self, oneof_name: &str) -> ::std::option::Option<&'static str> {
    match oneof_name {
      "result" => {
        if let Some(TaskUpdate_Result::TaskResult(ref val)) = self.result {
          return Some("task_result");
        }
        if let Some(TaskUpdate_Result::TaskError(ref val)) = self.result {
          return Some("task_error");
        }
        if let Some(TaskUpdate_Result::TaskCancelled(ref val)) = self.result {
          return Some("task_cancelled");
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
      "task_id" => {
        ::pb_jelly::reflection::FieldMut::Value(self.task_id.get_or_insert_with(::std::default::Default::default))
      }
      "status" => {
        ::pb_jelly::reflection::FieldMut::Value(&mut self.status)
      }
      "worker_status" => {
        ::pb_jelly::reflection::FieldMut::Value(self.worker_status.get_or_insert_with(::std::default::Default::default))
      }
      "task_result" => {
        match self.result {
          Some(TaskUpdate_Result::TaskResult(_)) => (),
          _ => {
            self.result = Some(TaskUpdate_Result::TaskResult(::std::default::Default::default()));
          },
        }
        if let Some(TaskUpdate_Result::TaskResult(ref mut val)) = self.result {
          return ::pb_jelly::reflection::FieldMut::Value(val);
        }
        unreachable!()
      }
      "task_error" => {
        match self.result {
          Some(TaskUpdate_Result::TaskError(_)) => (),
          _ => {
            self.result = Some(TaskUpdate_Result::TaskError(::std::default::Default::default()));
          },
        }
        if let Some(TaskUpdate_Result::TaskError(ref mut val)) = self.result {
          return ::pb_jelly::reflection::FieldMut::Value(val);
        }
        unreachable!()
      }
      "task_cancelled" => {
        match self.result {
          Some(TaskUpdate_Result::TaskCancelled(_)) => (),
          _ => {
            self.result = Some(TaskUpdate_Result::TaskCancelled(::std::default::Default::default()));
          },
        }
        if let Some(TaskUpdate_Result::TaskCancelled(ref mut val)) = self.result {
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

/// Message sent from workers to Galactus
#[derive(Clone, Debug, PartialEq)]
pub struct WorkerMessage {
  /// The worker's ID
  pub worker_id: ::std::option::Option<WorkerID>,
  /// Sent when the worker first registers so the manager knows what it can do
  /// Sent when the worker has an update on a task
  pub message: ::std::option::Option<WorkerMessage_Message>,
}
#[derive(Clone, Debug, PartialEq)]
pub enum WorkerMessage_Message {
  WorkerRegistration(WorkerRegistration),
  TaskUpdate(TaskUpdate),
}
impl ::std::default::Default for WorkerMessage {
  fn default() -> Self {
    WorkerMessage {
      worker_id: ::std::default::Default::default(),
      message: None,
    }
  }
}
::lazy_static::lazy_static! {
  pub static ref WorkerMessage_default: WorkerMessage = WorkerMessage::default();
}
impl ::pb_jelly::Message for WorkerMessage {
  fn descriptor(&self) -> ::std::option::Option<::pb_jelly::MessageDescriptor> {
    Some(::pb_jelly::MessageDescriptor {
      name: "WorkerMessage",
      full_name: "galactus.WorkerMessage",
      fields: &[
        ::pb_jelly::FieldDescriptor {
          name: "worker_id",
          full_name: "galactus.WorkerMessage.worker_id",
          index: 0,
          number: 1,
          typ: ::pb_jelly::wire_format::Type::LengthDelimited,
          label: ::pb_jelly::Label::Optional,
          oneof_index: None,
        },
        ::pb_jelly::FieldDescriptor {
          name: "worker_registration",
          full_name: "galactus.WorkerMessage.worker_registration",
          index: 1,
          number: 2,
          typ: ::pb_jelly::wire_format::Type::LengthDelimited,
          label: ::pb_jelly::Label::Optional,
          oneof_index: Some(0),
        },
        ::pb_jelly::FieldDescriptor {
          name: "task_update",
          full_name: "galactus.WorkerMessage.task_update",
          index: 2,
          number: 3,
          typ: ::pb_jelly::wire_format::Type::LengthDelimited,
          label: ::pb_jelly::Label::Optional,
          oneof_index: Some(0),
        },
      ],
      oneofs: &[
        ::pb_jelly::OneofDescriptor {
          name: "message",
        },
      ],
    })
  }
  fn compute_size(&self) -> usize {
    let mut size = 0usize;
    if let Some(ref val) = self.worker_id {
      size += ::pb_jelly::helpers::compute_size_field::<WorkerID>(val, 1, ::pb_jelly::wire_format::Type::LengthDelimited);
    }
    if let Some(WorkerMessage_Message::WorkerRegistration(ref val)) = self.message {
      size += ::pb_jelly::helpers::compute_size_field::<WorkerRegistration>(val, 2, ::pb_jelly::wire_format::Type::LengthDelimited);
    }
    if let Some(WorkerMessage_Message::TaskUpdate(ref val)) = self.message {
      size += ::pb_jelly::helpers::compute_size_field::<TaskUpdate>(val, 3, ::pb_jelly::wire_format::Type::LengthDelimited);
    }
    size
  }
  fn serialize<W: ::pb_jelly::PbBufferWriter>(&self, w: &mut W) -> ::std::io::Result<()> {
    if let Some(ref val) = self.worker_id {
      ::pb_jelly::helpers::serialize_field::<W, WorkerID>(w, val, 1, ::pb_jelly::wire_format::Type::LengthDelimited)?;
    }
    if let Some(WorkerMessage_Message::WorkerRegistration(ref val)) = self.message {
      ::pb_jelly::helpers::serialize_field::<W, WorkerRegistration>(w, val, 2, ::pb_jelly::wire_format::Type::LengthDelimited)?;
    }
    if let Some(WorkerMessage_Message::TaskUpdate(ref val)) = self.message {
      ::pb_jelly::helpers::serialize_field::<W, TaskUpdate>(w, val, 3, ::pb_jelly::wire_format::Type::LengthDelimited)?;
    }
    Ok(())
  }
  fn deserialize<B: ::pb_jelly::PbBufferReader>(&mut self, mut buf: &mut B) -> ::std::io::Result<()> {
    while let Some((field_number, typ)) = ::pb_jelly::wire_format::read(&mut buf)? {
      match field_number {
        1 => {
          let val = ::pb_jelly::helpers::deserialize_length_delimited::<B, WorkerID>(buf, typ, "WorkerMessage", 1)?;
          self.worker_id = Some(val);
        }
        2 => {
          let val = ::pb_jelly::helpers::deserialize_length_delimited::<B, WorkerRegistration>(buf, typ, "WorkerMessage", 2)?;
          self.message = Some(WorkerMessage_Message::WorkerRegistration(val));
        }
        3 => {
          let val = ::pb_jelly::helpers::deserialize_length_delimited::<B, TaskUpdate>(buf, typ, "WorkerMessage", 3)?;
          self.message = Some(WorkerMessage_Message::TaskUpdate(val));
        }
        _ => {
          ::pb_jelly::skip(typ, &mut buf)?;
        }
      }
    }
    Ok(())
  }
}
impl ::pb_jelly::Reflection for WorkerMessage {
  fn which_one_of(&self, oneof_name: &str) -> ::std::option::Option<&'static str> {
    match oneof_name {
      "message" => {
        if let Some(WorkerMessage_Message::WorkerRegistration(ref val)) = self.message {
          return Some("worker_registration");
        }
        if let Some(WorkerMessage_Message::TaskUpdate(ref val)) = self.message {
          return Some("task_update");
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
      "worker_id" => {
        ::pb_jelly::reflection::FieldMut::Value(self.worker_id.get_or_insert_with(::std::default::Default::default))
      }
      "worker_registration" => {
        match self.message {
          Some(WorkerMessage_Message::WorkerRegistration(_)) => (),
          _ => {
            self.message = Some(WorkerMessage_Message::WorkerRegistration(::std::default::Default::default()));
          },
        }
        if let Some(WorkerMessage_Message::WorkerRegistration(ref mut val)) = self.message {
          return ::pb_jelly::reflection::FieldMut::Value(val);
        }
        unreachable!()
      }
      "task_update" => {
        match self.message {
          Some(WorkerMessage_Message::TaskUpdate(_)) => (),
          _ => {
            self.message = Some(WorkerMessage_Message::TaskUpdate(::std::default::Default::default()));
          },
        }
        if let Some(WorkerMessage_Message::TaskUpdate(ref mut val)) = self.message {
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

/// Message sent from Galactus to workers
#[derive(Clone, Debug, PartialEq)]
pub struct GalactusWorkerMessage {
  pub message: ::std::option::Option<GalactusWorkerMessage_Message>,
}
#[derive(Clone, Debug, PartialEq)]
pub enum GalactusWorkerMessage_Message {
  TaskAssignment(TaskAssignment),
}
impl ::std::default::Default for GalactusWorkerMessage {
  fn default() -> Self {
    GalactusWorkerMessage {
      message: None,
    }
  }
}
::lazy_static::lazy_static! {
  pub static ref GalactusWorkerMessage_default: GalactusWorkerMessage = GalactusWorkerMessage::default();
}
impl ::pb_jelly::Message for GalactusWorkerMessage {
  fn descriptor(&self) -> ::std::option::Option<::pb_jelly::MessageDescriptor> {
    Some(::pb_jelly::MessageDescriptor {
      name: "GalactusWorkerMessage",
      full_name: "galactus.GalactusWorkerMessage",
      fields: &[
        ::pb_jelly::FieldDescriptor {
          name: "task_assignment",
          full_name: "galactus.GalactusWorkerMessage.task_assignment",
          index: 0,
          number: 1,
          typ: ::pb_jelly::wire_format::Type::LengthDelimited,
          label: ::pb_jelly::Label::Optional,
          oneof_index: Some(0),
        },
      ],
      oneofs: &[
        ::pb_jelly::OneofDescriptor {
          name: "message",
        },
      ],
    })
  }
  fn compute_size(&self) -> usize {
    let mut size = 0usize;
    if let Some(GalactusWorkerMessage_Message::TaskAssignment(ref val)) = self.message {
      size += ::pb_jelly::helpers::compute_size_field::<TaskAssignment>(val, 1, ::pb_jelly::wire_format::Type::LengthDelimited);
    }
    size
  }
  fn serialize<W: ::pb_jelly::PbBufferWriter>(&self, w: &mut W) -> ::std::io::Result<()> {
    if let Some(GalactusWorkerMessage_Message::TaskAssignment(ref val)) = self.message {
      ::pb_jelly::helpers::serialize_field::<W, TaskAssignment>(w, val, 1, ::pb_jelly::wire_format::Type::LengthDelimited)?;
    }
    Ok(())
  }
  fn deserialize<B: ::pb_jelly::PbBufferReader>(&mut self, mut buf: &mut B) -> ::std::io::Result<()> {
    while let Some((field_number, typ)) = ::pb_jelly::wire_format::read(&mut buf)? {
      match field_number {
        1 => {
          let val = ::pb_jelly::helpers::deserialize_length_delimited::<B, TaskAssignment>(buf, typ, "GalactusWorkerMessage", 1)?;
          self.message = Some(GalactusWorkerMessage_Message::TaskAssignment(val));
        }
        _ => {
          ::pb_jelly::skip(typ, &mut buf)?;
        }
      }
    }
    Ok(())
  }
}
impl ::pb_jelly::Reflection for GalactusWorkerMessage {
  fn which_one_of(&self, oneof_name: &str) -> ::std::option::Option<&'static str> {
    match oneof_name {
      "message" => {
        if let Some(GalactusWorkerMessage_Message::TaskAssignment(ref val)) = self.message {
          return Some("task_assignment");
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
      "task_assignment" => {
        match self.message {
          Some(GalactusWorkerMessage_Message::TaskAssignment(_)) => (),
          _ => {
            self.message = Some(GalactusWorkerMessage_Message::TaskAssignment(::std::default::Default::default()));
          },
        }
        if let Some(GalactusWorkerMessage_Message::TaskAssignment(ref mut val)) = self.message {
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

