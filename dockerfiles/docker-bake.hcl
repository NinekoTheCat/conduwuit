target "rocksdb-base" {
    dockerfile="dockerfiles/libs/Dockerfile.rocksdb"
}
target "rocksdb-dynamic" {
    inherits = ["rocksdb-base"]
    tags = ["rocksdb-compiled-shared:v9.9.3"]
        args = {
        ROCKS_DB_TARGET = "shared_lib"
    }
}

target "rocksdb-static" {
    inherits = ["rocksdb-base"]
    tags = ["rocksdb-compiled-static:v9.9.3"]
    args = {
        ROCKS_DB_TARGET = "static_lib"
    }
}


target "static-base" {
    dockerfile="dockerfiles/static/Dockerfile.base"
    context = "."
    output = ["type=cacheonly"]
    contexts = {
        rocksdb = "target:rocksdb-dynamic"
    }
}

target "static-base-profile-test"{
    inherits = ["static-base"]
    args = {
        CARGO_PROFILE = "test-max-perf"
    }
}
group "statics" {
    targets = ["static-base-test"]
}

target "static-base-test" {
    inherits = ["static-base-profile-test","keys"]
    dockerfile = "dockerfiles/static/Dockerfile.test-main"
    context = "."
    contexts = {
        base = "target:static-base-profile-test"
        keys = "target:keys"
    }
}


target "complement" {
  inherits = ["base-test","keys"]
  tags = ["conduwuit-complement:latest"]
}

target "keys" {
    dockerfile = "dockerfiles/Dockerfile.keys"
    context ="."
    output = ["type=cacheonly"]
}

target "dyn-base" {
    dockerfile="dockerfiles/dynamic/Dockerfile.base"
    context = "."
    output = ["type=cacheonly"]
}

target "dyn-test" {
    inherits = ["dyn-base"]
        args = {
        CARGO_PROFILE = "test"
    }
    output = ["type=image"]
}

group "dynamics" {
    targets=[""]
}