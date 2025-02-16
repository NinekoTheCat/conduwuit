target "rocksdb" {
    dockerfile="dockerfiles/libs/Dockerfile.rocksdb"
    tags = ["rocksdb-compiled:v9.9.3"]
}


target "static-base" {
    dockerfile="dockerfiles/static/Dockerfile.base"
    context = "."
    output = ["type=cacheonly"]
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