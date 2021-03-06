#
#
#
V ?= @
-include ../common.mk


# ---------------------------------------

# - ACPICA can only be used on x86_64/amd64
ifeq ($(ARCH),amd64)
 USE_ACPICA ?= 1
else
 USE_ACPICA = 0
endif

-include BuildNum.$(ARCH).mk
BUILDNUM ?= 1

OBJDIR := .obj/$(ARCH)/
OBJS := $(addprefix $(OBJDIR),start.ao log_cfg.ao libmain.a)
IMGDIR := ../Graphics/.output/shared/

CARGO_FEATURES :=
ENV := RUST_VERSION="$(shell $(RUSTC) --version)"
ENV += TK_GITSPEC="$(shell git log -n 1 | head -n 1 | awk '{print $$2}')"
ENV += TK_VERSION="0.1"
ENV += TK_BUILD="$(shell hostname --fqdn):$(BUILDNUM)"

LINKFLAGS := -T Core/arch/$(ARCH)/link.ld -Map $(OBJDIR)map.txt -g
LINKFLAGS += -z max-page-size=0x1000	# Reduces binary size
LINKFLAGS += --gc-sections	# Prevents complaining about floating point methods
#LINKFLAGS += --print-gc-sections

EXCLUDE_FEATURES :=

ifeq ($(USE_ACPICA),1)
  OBJS += $(OBJDIR)libacpica.a
  CARGO_FEATURES += kernel/acpica
endif

XARGO := RUST_TARGET_PATH=$(abspath targets) $(XARGO)

.PHONY: all dsm PHONY

all: bin/kernel-$(ARCH).bin
dsm: bin/kernel-$(ARCH).bin.dsm

# 
# Final linking stage
# 
# TODO: Need to get the library list from cargo
bin/kernel-$(ARCH).bin: $(OBJS) Core/arch/$(ARCH)/link.ld
	@mkdir -p bin/
	@echo [LD] -o $@
	$V$(LD) -o $@ $(OBJS) --no-whole-archive $(LINKFLAGS)
ifeq ($(ARCH),amd64)
	$Vobjcopy $@ --only-keep-debug $@.debug
	$Vobjcopy $@ -F elf32-i386 $@.elf32
	$V$(TRIPLE)-strip -g $@.elf32
else
	#$V$(TRIPLE)-strip $@
endif
	@echo "BUILDNUM := $$(( $(BUILDNUM) + 1 ))" > BuildNum.$(ARCH).mk

#
# Disassembly
#
%.dsm: %
	@echo "[OBJDUMP] >" $*.dsm
	$V$(OBJDUMP) -s -C -S $* > $*.dsm

#
# Invoke xargo to build the main part of the kernel
# - This includes all modules enabled in ./Cargo.toml
#
$(OBJDIR)libmain.a: PHONY targets/target-k-$(ARCH).json $(IMGDIR)logo.rs $(IMGDIR)panic.rs
	@echo "[XARGO] . > $@"
	$V$(ENV) $(XARGO) build --target=target-k-$(ARCH) --release --features "$(CARGO_FEATURES)"
	@mkdir -p $(dir $@)
	@cp target/target-k-$(ARCH)/release/libmain.a $@
$(patsubst %,$(OBJDIR)lib%.a,$(MODS)): $(OBJDIR)libmain.a

targets/target-k-$(ARCH).json: Core/arch/$(ARCH)/target.json
	@mkdir -p $(dir $@)
	$Vcp $< $@

# ACPICA
$(OBJDIR)libacpica.a: ../acpica/Makefile $(wildcard ../acpica/patches/source/include/platform/*)
	make -C ../acpica TRIPLE=$(TRIPLE) ARCH=$(ARCH) srcdir
	make -C ../acpica TRIPLE=$(TRIPLE) ARCH=$(ARCH)
	cp ../acpica/acpica-$(TRIPLE).a $@

$(OBJDIR)%.ao: Core/arch/$(ARCH)/%.asm
	@echo [AS] -o $@
	@mkdir -p $(dir $@)
	$Vnasm -o $@ $< -f elf64 -MD $@.d -MP
$(OBJDIR)%.ao: Core/arch/$(ARCH)/%.S
	@echo [AS] -o $@
	@mkdir -p $(dir $@)
	$V$(CC) -o $@ -c $<
# Log configuration
$(OBJDIR)log_cfg.ao: Core/log_cfg.S
	@echo [AS] -o $@
	@mkdir -p $(dir $@)
	$V$(CC) -o $@ -c $<

$(IMGDIR)%.rs: ../Graphics/Makefile
	cd ../Graphics && make $(@:../Graphics/%=%)
