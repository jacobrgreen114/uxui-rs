# Uxui-rs
Uxui is a crossplatform UI framework written targeting desktop platforms.

## Speed
Uxui is a retained mode UI framework and designed around minimizing as much dynamic dispatch as possible.

## State
As of right now, Uxui is in a very early stage of development. It is not ready for production use.
Currently, due to the lack of inheritance, 'Components' are impemented purely using dyn traits. 
This results in each component having to handle its own sizing and event propagation to its children.
This is not ideal, and will be changed in the future once a better solutions is decided upon.