#pragma once

#include "rocksdb/db.h"

using ROCKSDB_NAMESPACE::DBOptions;

std::shared_ptr<DBOptions> new_db_options();
