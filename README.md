# 🦀 RustFetch

Um clone do `neofetch` escrito em Rust - uma ferramenta de informações do sistema rápida e colorida.

## ✨ Funcionalidades

- **Informações do Sistema**: Mostra informações essenciais do sistema operacional
- **Performance**: Escrito em Rust para máxima velocidade
- **Colorido**: Interface visual atrativa com cores
- **Multiplataforma**: Funciona em Linux, macOS e Windows
- **Leve**: Executável pequeno e rápido

## 📋 Informações Exibidas

- **Usuario@Hostname**: Nome do usuário e hostname do sistema
- **OS**: Sistema operacional e distribuição
- **Kernel**: Versão do kernel
- **Uptime**: Tempo de atividade do sistema
- **Shell**: Shell atual sendo usado
- **CPU**: Modelo do processador e uso atual
- **Memória**: Uso de memória RAM
- **Disco**: Uso do disco no ponto de montagem raiz (/)
- **Cores**: Paleta de cores do terminal

## 🚀 Instalação

### Pré-requisitos

- Rust (versão 1.60 ou superior)
- Cargo (incluído com Rust)

### Compilação

```bash
# Clone o repositório
git clone https://github.com/leandroFeitoza81/rustfetch.git
cd rustfetch

# Compile em modo release
cargo build --release

# Execute
./target/release/rustfetch
```

### Instalação Global

```bash
# Compile e instale
cargo install --path .

# Agora você pode executar de qualquer lugar
rustfetch
```

## 🎯 Uso

Simplesmente execute o comando:

```bash
rustfetch
```

## 🔧 Dependências

- `sysinfo` - Para obter informações do sistema
- `whoami` - Para informações do usuário
- `colored` - Para colorir a saída

## 🆚 Comparação com Neofetch

| Característica | RustFetch | Neofetch |
|----------------|-----------|----------|
| Linguagem      | Rust      | Bash     |
| Velocidade     | ⚡ Muito rápida | 🐌 Lenta |
| Memória        | 📦 Baixo uso | 💾 Alto uso |
| Dependências   | 📋 Poucas | 📚 Muitas |
| Personalização | 🔧 Básica | 🎨 Extensa |

## 🛠️ Desenvolvimento

### Estrutura do Projeto

```
rustfetch/
├── src/
│   └── main.rs          # Código principal
├── Cargo.toml           # Configuração do Cargo
└── README.md           # Este arquivo
```

### Contribuindo

1. Faça um fork do projeto
2. Crie uma branch para sua feature (`git checkout -b feature/AmazingFeature`)
3. Commit suas mudanças (`git commit -m 'Add some AmazingFeature'`)
4. Faça push para a branch (`git push origin feature/AmazingFeature`)
5. Abra um Pull Request

## 📝 Licença

Este projeto está licenciado sob a MIT License - veja o arquivo [LICENSE](LICENSE) para detalhes.

## 🙏 Reconhecimentos

- [Neofetch](https://github.com/dylanaraps/neofetch) - Inspiração original
- [sysinfo](https://github.com/GuillaumeGomez/sysinfo) - Biblioteca de informações do sistema
- [whoami](https://github.com/libcala/whoami) - Informações do usuário
- [colored](https://github.com/colored-rs/colored) - Cores no terminal

## 📊 Exemplo de Saída

```
    [usuario]-@[hostname]
    ─────────────────────
    OS: Arch Linux
    Kernel: 6.6.87.2-microsoft-standard-WSL2
    Uptime: 2 hours, 45 mins
    Shell: zsh
    CPU: 12th Gen Intel(R) Core(TM) i7-1265U (0.0%)
    Memory: 0.6 GiB / 15.5 GiB (4.0%)
    Disk (/): 54.2 GiB / 1006.9 GiB (5.4%)

    Colors: ████████████████████████
```
