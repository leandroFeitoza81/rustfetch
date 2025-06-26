# RustFetch Makefile

# Variáveis
CARGO = cargo
TARGET_DIR = target/release
BINARY_NAME = rustfetch
INSTALL_DIR = /usr/local/bin

# Targets
.PHONY: all build release clean install uninstall run test help

# Target padrão
all: release

# Compilação em modo debug
build:
	@echo "Compilando em modo debug..."
	$(CARGO) build

# Compilação em modo release (otimizada)
release:
	@echo "Compilando em modo release..."
	$(CARGO) build --release

# Limpar arquivos de compilação
clean:
	@echo "Limpando arquivos de compilação..."
	$(CARGO) clean

# Instalar o binário no sistema
install: release
	@echo "Instalando $(BINARY_NAME) em $(INSTALL_DIR)..."
	sudo cp $(TARGET_DIR)/$(BINARY_NAME) $(INSTALL_DIR)/
	@echo "$(BINARY_NAME) instalado com sucesso!"

# Desinstalar o binário do sistema
uninstall:
	@echo "Desinstalando $(BINARY_NAME)..."
	sudo rm -f $(INSTALL_DIR)/$(BINARY_NAME)
	@echo "$(BINARY_NAME) desinstalado com sucesso!"

# Executar o programa
run: release
	@echo "Executando $(BINARY_NAME)..."
	./$(TARGET_DIR)/$(BINARY_NAME)

# Executar testes
test:
	@echo "Executando testes..."
	$(CARGO) test

# Verificar código (linting)
check:
	@echo "Verificando código..."
	$(CARGO) check
	$(CARGO) clippy

# Formatar código
format:
	@echo "Formatando código..."
	$(CARGO) fmt

# Mostrar ajuda
help:
	@echo "RustFetch - Makefile"
	@echo ""
	@echo "Targets disponíveis:"
	@echo "  all        - Compilar em modo release (padrão)"
	@echo "  build      - Compilar em modo debug"
	@echo "  release    - Compilar em modo release"
	@echo "  clean      - Limpar arquivos de compilação"
	@echo "  install    - Instalar o binário no sistema"
	@echo "  uninstall  - Desinstalar o binário do sistema"
	@echo "  run        - Compilar e executar o programa"
	@echo "  test       - Executar testes"
	@echo "  check      - Verificar código (linting)"
	@echo "  format     - Formatar código"
	@echo "  help       - Mostrar esta ajuda"
