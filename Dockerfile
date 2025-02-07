# Usa la imagen oficial de Rust como base
FROM rust:1.75 as builder

# Establece el directorio de trabajo dentro del contenedor
WORKDIR /usr/src/app

# Copia el archivo Cargo.toml y Cargo.lock y descarga dependencias
COPY ./path/to/Cargo.toml ./path/to/Cargo.lock /usr/src/app/
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release && rm -rf src

# Copia el resto del código fuente
COPY . .

# Compila la aplicación en modo release
RUN cargo build --release

# Segunda etapa: imagen más liviana con solo el binario
FROM debian:bullseye-slim

# Establece el directorio de trabajo
WORKDIR /usr/src/app

# Copia el binario compilado desde la primera imagen
COPY --from=builder /usr/src/app/target/release/tu_aplicacion ./actix_backend

# Exponer el puerto en el que corre Actix
EXPOSE 8080

# Comando para ejecutar la aplicación
CMD ["./actix_backend"]
