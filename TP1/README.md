# [_AlGlobo.com_](https://alglobo.herokuapp.com/)

Microservicio de AlGlobo que expone una REST API para realizar reservas. Escrito en **Rust**.

## Uso :computer:

Una vez que el servicio se encuentre corriendo (ya sea en local o en un proveedor cloud), se expone la siguiente API:

- `GET /`: healthcheck básico (ping).
- `GET /metrics`: permite obtener métricas útiles sobre el servicio.
- `POST /request`: permite enviar una request, obteniendo un `id` (`uuid v4`) para hacer su seguimiento.
- `GET /request?id={id}`: permite consultar el estado de una request con `id = {id}`.

Para más información sobre el uso de cada uno de estos endpoints y de la API en general, así como de nuestra [interfaz gráfica web](https://mauro7x.github.io/concurrentes/), se encuentra disponible nuestro [Manual de Usuario](./docs/ManualDeUsuario.pdf).

## Desarrollo local :wrench:

### Cargo

Se utiliza `cargo` como gestor de paquetes de Rust. En caso de querer más información, se puede ejecutar `cargo help` para mostrar la lista de comandos disponibles, y luego:

```bash
$ cargo help <command>
```

Para obtener mayor información sobre cualquiera de ellos.

### Proyecto

_En cualquiera de los siguientes comandos, reemplazar `<bin>` por `part1` o `part2`, según se desee correr la primera parte o la segunda (respectivamente)._

- Para **generar documentación** del proyecto (opcionalmente, especificar un binario):
  ```bash
  $ cargo doc [--bin <bin>]
  ```
- Para **compilar** el proyecto (opcionalmente, especificar un binario):
  ```bash
  $ cargo build [--bin <bin>] [--release]
  ```
- Para compilar **y correr** el proyecto:
  ```bash
  $ cargo run --bin <bin> [--release]
  ```
  La parte 1 recibe el archivo de las requests (en formato `csv`) como un argumento opcional.
  ```bash
  $ cargo run --bin part1 ./custom_requests.csv
  ```
- Para correr las **pruebas unitarias**:
  ```bash
  $ cargo test [--bin <bin>]
  ```
- Para formatear el proyecto:
  ```bash
  $ cargo fmt
  ```
- Para correr el **linter**:
  ```bash
  $ cargo clippy
  ```
- Para limpiar el ambiente:
  ```bash
  $ cargo clean
  ```

### Configuración

Pueden configurarse los principales parámetros del sistema así como las **aerolineas** y el **hotel** desde los archivos de configuración ([`/config`](./config)).

## Documentación :books:

A continuación se lista documentación relevante (disponible en nuestra sección [`/docs`](./docs)):

- [Enunciado](./docs/Enunciado.md)
- [Informe](./docs/Informe.pdf)
- [Manual de Usuario](./docs/ManualDeUsuario.pdf)
