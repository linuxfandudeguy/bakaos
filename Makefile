# -------- config --------
BIN_NAME := bakashell
TARGET   := target/release/$(BIN_NAME)
IMAGE    := bakaos

# -------- defaults --------
.PHONY: all
all: build

# -------- build rust binary --------
.PHONY: build
build:
	cargo build --release
	strip $(TARGET) || true

# -------- run shell locally --------
.PHONY: run
run: build
	./$(TARGET)

# -------- docker image --------
.PHONY: docker
docker: build
	sudo docker build -t $(IMAGE) .

# -------- run docker --------
.PHONY: run-os
run-os: docker
	sudo docker run -it --rm $(IMAGE)

# -------- clean --------
.PHONY: clean
clean:
	cargo clean
	rm -f $(TARGET)

