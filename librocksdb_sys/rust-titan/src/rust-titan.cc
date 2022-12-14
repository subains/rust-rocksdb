#include "rust-titan.h"
#include "rust-titan/src/lib.rs.h"

using ROCKSDB_NAMESPACE::DBOptions;

std::shared_ptr<DBOptions> new_db_options() {
  return std::make_shared<DBOptions>();
}

