# Stage 39 Console IO Architecture

## New module

```text
reta_architecture/console_io.py
```

## Bundle

```text
ConsoleIOMorphismBundle
```

## Capsule ownership

Primary capsule:

```text
OutputRenderingCapsule
```

Secondary capsule:

```text
InputPromptCapsule
```

Compatibility capsule:

```text
CompatibilityCapsule / libs/center.py
```

## Morphisms

```text
load_reta_prompt_help
load_reta_help
print_reta_prompt_help
print_reta_help
text_wrap_runtime
cli_output
debug_pair
debug_value
chunks
unique_everseen
DefaultOrderedDict
```

## Old API preserved

```text
center.retaPromptHilfe
center.retaHilfe
center.getTextWrapThings
center.cliout
center.x
center.alxp
center.chunks
center.unique_everseen
center.DefaultOrderedDict
```

## Natural transformations

```text
CenterConsoleIOToArchitectureTransformation
ConsoleIOOutputValidationTransformation
```

## Commutative diagrams

```text
CenterConsoleIOCompatibilitySquare
ConsoleIOOutputValidationSquare
```

## Refactor law

```text
ActivatedConsoleIOLaw
```

The law prevents future stages from reintroducing console/help/wrapping logic as
owned implementation inside `libs/center.py`.
