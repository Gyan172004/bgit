# Derived from https://github.com/juspay/hyperswitch/blob/main/Makefile
# = Parameters
# Override envars using -e

#
# = Common
#

# Checks two given strings for equality.
eq = $(if $(or $(1),$(2)),$(and $(findstring $(1),$(2)),\
                                $(findstring $(2),$(1))),1)

#
# = Targets
#

.PHONY : \
	doc \
	fmt \
	clippy \
	test \
	audit \
	git.sync \
	build \
	push \
	shell \
	run \
	start \
	stop \
	rm \
	release \
	coverage

# Compile application for running on local machine
#
# Usage :
#	make build

build :
	cargo build

# Generate crates documentation from Rust sources.
#
# Usage :
#	make doc [private=(yes|no)] [open=(yes|no)] [clean=(no|yes)]

doc :
ifeq ($(clean),yes)
	@rm -rf target/doc/
endif
	cargo doc --all-features --no-deps\
		$(if $(call eq,$(private),no),,--document-private-items) \
		$(if $(call eq,$(open),no),,--open)

# Format Rust sources with rustfmt.
#
# Usage :
#	make fmt [fix=(no|yes)]

fmt :
	cargo fmt --all $(if $(call eq,$(fix),yes),,-- --check)

# Lint Rust sources with Clippy.
#
# Usage :
#	make clippy

clippy :
	cargo clippy --all-features --all-targets -- -D warnings

# Run Rust tests of project.
#
# Usage :
#	make test

test :
	cargo test --all-features

# Run format clippy test and tests.
#
# Usage :
#	make precommit
precommit : fmt clippy test

coverage:
	cargo llvm-cov clean --workspace --html --output-dir=coverage
	cargo llvm-cov --all-features --workspace --no-clean --html --output-dir=coverage --open

release:
	cargo smart-release bgit --execute --update-crates-index
