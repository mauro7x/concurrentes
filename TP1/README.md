# _AlGlobo.com_

_TODO: Descripción._

## Pre-requisitos :wrench:

_TODO_

## Uso :computer:

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
    $ cargo run [--bin <bin>] [--release]
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

## Documentación :books:

A continuación se lista documentación relevante (disponible en nuestra sección [`/docs`](./docs)).

-   [Enunciado](./docs/Enunciado.md).
-   Informe. _TODO_
