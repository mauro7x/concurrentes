# [_AlGlobo.com_](https://alglobo.herokuapp.com/)

_TODO: Descripción._

## Uso :computer:

Una vez que el servicio se encuentre corriendo (ya sea en local o en un proveedor cloud), se expone la siguiente API:

* `GET /`: Healthcheck básico (ping).
* `GET /metrics`: Permite obtener métricas útiles sobre el servicio.
* `POST /request`: Permite enviar una request, obteniendo un `id` (`uuid v4`) para hacer su seguimiento. La misma debe tener el siguiente formato:
  ```rust
  {
    origin: String,   // IATA del Aeropuerto origen
    destiny: String,  // IATA del Aeropuerto destino
    airline: String,  // Nombre de la Aerolinea
    package: bool     // Si se debe reservar hotel
  }
  ```
  Por ejemplo:
  ```json
  {
    "origin": "EZE",
    "destiny": "JFK",
    "airline": "American Airlines",
    "package": true
  }
  ```
* `GET /request?id={id}`: Permite consultar el estado de una request con `id = {id}`. La respuesta tiene el siguiente formato:
  ```rust
  {
    id: String,       // uuid v4 de la request
    airline: String,
    origin: String,
    destiny: String,
    package: bool,
    status: String    // PENDING o COMPLETED
  }
  ```
  Por ejemplo:
  ```json
  {
    "id": "da7c49b8-66e6-491b-961e-5d41712e2aa0",
    "airline": "Delta Air Lines",
    "origin": "MAD",
    "destiny": "CDG",
    "package": true,
    "status": "COMPLETED"
  }
  ```
## Desarrollo local :wrench:

### Cargo

Se utiliza `cargo` como gestor de paquetes de Rust. En caso de querer más información, se puede ejecutar `cargo help` para mostrar la lista de comandos disponibles, y luego:

```bash
$ cargo help <command>
```

Para obtener mayor información sobre cualquiera de ellos.

### Proyecto

_En cualquiera de los siguientes comandos, reemplazar `<bin>` por `part1` o `part2`, según se desee correr la primera parte o la segunda (respectivamente)._

-   Para **compilar** el proyecto (opcionalmente, especificar un binario):
    ```bash
    $ cargo build [--bin <bin>] [--release]
    ```
-   Para compilar **y correr** el proyecto:
    ```bash
    $ cargo run --bin <bin> [--release]
    ```
-   Para correr las **pruebas unitarias**:
    ```bash
    $ cargo test [--bin <bin>]
    ```
-   Para formatear el proyecto:
    ```bash
    $ cargo fmt
    ```
-   Para correr el **linter**:
    ```bash
    $ cargo clippy
    ```
-   Para limpiar el ambiente:
    ```bash
    $ cargo clean
    ```

### Configuración

Pueden configurarse los principales parámetros del sistema así como las **aerolineas** y el **hotel** desde los archivos de configuración ([`/config`](./config)).

## Documentación :books:

A continuación se lista documentación relevante (disponible en nuestra sección [`/docs`](./docs)).

-   [Enunciado](./docs/Enunciado.md).
-   Informe: pdf (WIP) - [overleaf (lectura)](https://es.overleaf.com/read/jcbzxvndgwkm) - [overleaf (edición)](https://es.overleaf.com/6759942824nmrwbpvwvndm)
