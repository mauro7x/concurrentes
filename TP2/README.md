# _AlGlobo.com_ (Sistema de Pagos)

Sistema compuesto por múltiples entidades independientes que se comunican entre sí utilizando sockets UDP y/o TCP mediante protocolos previamente definidos para llevar a cabo el procesamiento de una cola de pagos compartida.

## Uso :computer:

**TL;DR:**

- Para levantar el sistema:
  ```bash
  $ ./run.sh [<N_REPLICAS>] [<PAYMENTS_FILE>] [<ACCOUNTS_FILE>]
  ```
  Donde `<N_REPLICAS>` es el número de instancias de AlGlobo a crear, `<PAYMENTS_FILE>` la ruta al archivo de pagos, y `<ACCOUNTS_FILE>` la ruta al archivo de cuentas del banco. El formato esperado de los archivos se puede encontrar en los ejemplos (`/examples`).
- **Con el sistema corriendo**, para escalar la cantidad de replicas de AlGlobo a un determinado **número final** de réplicas:
  ```bash
  $ ./scale.sh <N_REPLICAS>
  ```
- **Con el sistema corriendo**, para levantar el servicio de reintentos manual:
  ```bash
  $ ./run_manual.sh
  ```
- Para correr cualquier comando para cada servicio (util por ejemplo para correr **cargo clippy**):
  ```bash
  $ ./run_for_services.sh <COMMAND>
  ```
- Para limpiar la data creada por Docker:
  ```bash
  $ ./clean_docker.sh
  ```
  **(CUIDADO: Se hace un `docker system prune`!)**

### Orquestrador

Debido a que el sistema a modelar está compuesto por distintas entidades, se utilizó **Docker** para crear sus containers aislados y luego [**Docker Compose**](https://docs.docker.com/compose/) como coordinador para levantar el sistema.

### Servicios

Cada uno de los servicios está escrito en Rust, utilizando `cargo` como gestor de paquetes.

#### Comandos principales

Dentro de cada servicio, se pueden ejecutar los siguientes comandos útiles (tener en mente que es probable que el servicio necesite variables de entorno adicionales para su correcta inicialización):

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

En el caso del servicio **AlGlobo**, debe utilizarse el flag `--bin <bin_name>` para indicar el binario que se desea, que puede ser:

- **`app`** para levantar una replica de AlGlobo.
- **`manual`** para nuestro servicio manual de reintento de requests.

#### Más

En caso de querer más información, se puede ejecutar `cargo help` para mostrar la lista de comandos disponibles, y luego:

```bash
$ cargo help <command>
```

Para obtener mayor información sobre cualquiera de ellos.

### Scripts

De todas formas, no basta con Docker Compose, ya que además tenemos archivos comunes y compartidos entre distintos servicios, así como un setup inicial que hay que realizar (para crear directorios necesarios, etc).

Es por esto que se provee de un script de inicialización que se encarga de hacer todo lo necesario para que el sistema funcione correctamente.

### Configuración

El sistema es configurable mediante variables de entorno, que pueden encontrarse en:

- `.env`: variables de entorno principales y compartidas entre distintos servicios.
- `config/*.env`: variables de entorno para configurar cada servicio por separado.

## Documentación :books:

A continuación se lista documentación relevante (disponible en nuestra sección [`/docs`](./docs)):

- [Enunciado](./docs/Enunciado.md)
- Informe: _pdf_ (WIP)
