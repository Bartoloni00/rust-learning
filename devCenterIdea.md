# DevCenter

## Developer Workspace Automation for Linux & macOS

---

# Resumen

DevCenter es una aplicación open source diseñada para automatizar y centralizar entornos de desarrollo.

Su objetivo es reducir la fricción diaria que enfrentan los desarrolladores al cambiar de contexto entre proyectos, servicios, servidores, terminales y herramientas.

En lugar de ejecutar múltiples comandos manualmente o mantener decenas de aliases en archivos `.zshrc`, DevCenter permite definir entornos de trabajo reutilizables y lanzarlos desde una interfaz visual.

---

# Problema

Muchos desarrolladores mantienen flujos de trabajo dispersos entre:

* Alias de shell
* Scripts bash
* Configuraciones SSH
* Comandos Docker
* Servicios locales
* IDEs
* Terminales

Ejemplo común:

```bash
cd ~/projects/broker

sudo systemctl start mysql

code .

npm run dev

php artisan serve

ssh production-server
```

Estas tareas deben repetirse diariamente.

A medida que aumenta la cantidad de proyectos, la complejidad crece y el tiempo perdido en tareas repetitivas también.

---

# Visión

Convertir la preparación de un entorno de desarrollo en una acción de un solo clic.

Ejemplo:

```text
▶ Start Broker Environment
```

DevCenter ejecuta automáticamente:

* Apertura de VSCode
* Inicio de servicios necesarios
* Apertura de terminales
* Ejecución de comandos
* Conexión a recursos asociados

Todo desde una única interfaz.

---

# Filosofía del Producto

DevCenter NO busca ser:

* Un reemplazo de Docker
* Un reemplazo de SSH
* Un monitor de infraestructura
* Un dashboard empresarial

DevCenter busca ser:

> Un automatizador visual de entornos de desarrollo.

---

# Público Objetivo

### Inicial

* Desarrolladores Linux
* Backend Developers
* Full Stack Developers
* DevOps con múltiples proyectos locales

### Futuro

* Usuarios macOS
* Equipos pequeños
* Freelancers
* Consultores que trabajan con múltiples clientes

---

# Principios de Diseño

## Simplicidad

El usuario debe poder comenzar a utilizar DevCenter en pocos minutos.

## Automatización

Reducir tareas manuales repetitivas.

## Configuración Declarativa

Los entornos se describen mediante configuración.

## Open Source First

Toda funcionalidad debe ser compatible con una comunidad open source.

## Extensibilidad

Las configuraciones deben crecer sin necesidad de modificar código.

---

# Arquitectura Conceptual

La entidad principal es:

```text
Workflow
```

No:

```text
Servicio
```

No:

```text
Proyecto
```

No:

```text
Host
```

Todo gira alrededor de automatizar flujos de trabajo.

---

# Entidades Principales

## Profile

Agrupa distintos contextos de trabajo.

Ejemplo:

```text
Work
Personal
Client A
Client B
```

---

## Project

Representa un proyecto local.

Ejemplo:

```text
Broker
Telepagos
Legacy CakePHP
DevCenter
```

---

## Workflow

Conjunto de acciones ejecutadas secuencialmente.

Ejemplo:

```text
Start Broker Environment
Deploy Staging
Open Legacy Project
```

---

## Service

Servicios locales administrados por el sistema.

Ejemplo:

```text
MySQL
Apache
Redis
```

---

## Host

Servidores remotos asociados.

Ejemplo:

```text
Production
Staging
Development
```

---

# Configuración

Se utilizará una estructura distribuida.

```text
~/.config/devcenter/

profiles/
projects/
workflows/
hosts/
services/
```

Motivos:

* Escalable
* Fácil de versionar
* Compatible con Git
* Compatible con Open Source

---

# Ejemplo de Workflow

```yaml
name: Broker Environment

steps:

  - start_service:
      name: mysql

  - open_vscode:
      project: broker

  - open_terminal:
      path: ~/projects/broker
      command: npm run dev

  - open_terminal:
      path: ~/projects/broker
      command: php artisan serve
```

---

# Manejo de Errores

Los workflows son secuenciales.

Si un paso falla:

```text
✓ Open VSCode
✓ Start MySQL
✗ npm run dev
```

La ejecución se detiene inmediatamente.

El usuario recibe el error correspondiente.

No se ejecutan pasos posteriores.

---

# Estrategia de Permisos

Para operaciones privilegiadas:

```bash
systemctl start mysql
```

DevCenter utilizará reglas configurables de sudoers.

Objetivo:

* Evitar solicitudes repetidas de contraseña.
* Mantener seguridad.
* Permitir automatización fluida.

---

# Roadmap

---

# Versión 0.1

## Objetivo

Validar el concepto.

## Funcionalidades

* TUI en terminal
* Lista de proyectos
* Abrir VSCode
* Abrir terminal en directorio del proyecto

Ejemplo:

```text
Projects

Broker
Telepagos
DevCenter
```

---

# Versión 0.2

## Workflows básicos

Permitir:

```yaml
steps:
  - open_vscode
  - open_terminal
```

---

# Versión 0.3

## Comandos personalizados

Permitir:

```yaml
steps:
  - run:
      command: npm run dev
```

---

# Versión 0.4

## Servicios

Administración básica:

```text
Start
Stop
Restart
Status
```

Para:

* Apache
* MySQL
* Redis

---

# Versión 0.5

## Hosts SSH

Configuración de servidores.

Acciones:

```text
Connect
```

Abriendo la terminal predeterminada del sistema.

---

# Versión 0.6

## Profiles

Separación de entornos.

Ejemplo:

```text
Work
Personal
Client A
```

---

# Versión 0.7

## Editor Visual

Crear:

* Proyectos
* Workflows
* Hosts
* Servicios

Sin editar YAML manualmente.

---

# Versión 0.8

## Aplicación gráfica

Migración desde TUI hacia GUI nativa.

Tecnología candidata:

* Rust
* Iced

---

# Versión 1.0

## Lanzamiento Público

Incluye:

* Linux estable
* Instalador .deb
* Documentación completa
* GitHub público
* Sistema de workflows
* Gestión de perfiles
* Gestión de proyectos
* Gestión de hosts
* Gestión de servicios

---

# Funcionalidades Futuras

## Detección automática de stack

Detectar:

```text
Cargo.toml
package.json
composer.json
docker-compose.yml
```

Y reconocer automáticamente:

```text
Rust
Node
Laravel
Docker
```

---

## Workflows paralelos

Ejemplo:

```yaml
parallel:

  - npm run dev

  - php artisan serve
```

---

## Integración Docker

Administración básica:

```text
Start
Stop
Status
```

---

## Soporte macOS

Compatibilidad completa.

---

## Marketplace de Workflows

Compartir configuraciones entre desarrolladores.

---

# Métrica de Éxito

Un desarrollador debería poder pasar de:

```bash
15 comandos manuales
```

a:

```text
1 click
```

para comenzar a trabajar en cualquier proyecto.

Ese es el objetivo principal de DevCenter.
