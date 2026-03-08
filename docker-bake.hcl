variable "VERSION" {
    default = "test"
}

variable "PLATFORMS" {
    # Drop 32-bit support
    # Work around buildx quirks
    default = [
        # "linux/386",
        "linux/amd64",
        # "linux/arm/v6",
        # "linux/arm/v7",
        "linux/arm64/v8",
        # "linux/ppc64le",
        # "linux/riscv64",
        # "linux/s390x",
    ]
}

variable "PRODUCTION" {
    default = [
        "kirill",
    ]
}

variable "TEST" {
    default = [
        "test-kirill",
    ]
}

group "production" {
    targets = PRODUCTION
}

group "test" {
    targets = TEST
}

group "all" {
    targets = concat(PRODUCTION, TEST)
}

target "kirill" {
    platforms = PLATFORMS
    tags = [
        "n4jm4/kirill:${VERSION}-alpine-linux-3.23",
        "n4jm4/kirill:${VERSION}-alpine-linux",
        "n4jm4/kirill:alpine-linux-3.23",
        "n4jm4/kirill:alpine-linux",
        "n4jm4/kirill",
    ]
}

target "test-kirill" {
    platforms = PLATFORMS
    tags = [
        "n4jm4/rockhopper:test-kirill",
    ]
}
