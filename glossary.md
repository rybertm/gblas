# Glossary

## Semiring

(From ChatGPT)
Algebraic structure that generalizes the concept of a ring but without the necessity of additive inverses (negative elements).
It consists of a set equipped with two binary operations, typically called addition and multiplication, that satisfy certain properties.

## Domain

The set of valid values that can be stored in either matrices or vectors is referred to as their domain.

## Info

The values stored in these matrices correspond to attributes (often weights) of edges in the graph.
Likewise, information about vertices in a graph are stored in vectors.

Matrices are usually sparse because the lack of an edge between two vertices means that nothing is stored at the corresponding location in the matrix.
Vectors may be sparse or dense, or they may start out sparse and become dense as algorithms traverse the graphs.

## Limitations

The mathematical formalism for graph operations in the language of linear algebra often assumes that we can operate in the field of real numbers.
However, the GraphBLAS C binding is designed for implementation on computers, which by necessity have a finite number of bits to represent numbers.
Therefore, we require a conforming implementation to use floating point numbers such as those defined by the IEEE-754 standard (both single- and double-precision) wherever real numbers need to be represented.
The practical implications of these finite precision numbers is that the result of a sequence of computations may vary from one execution to the next as the grouping of operands (because of associativity) within the operations changes.
While techniques are known to reduce these effects, we do not require or even expect an implementation to use them as they may add considerable overhead.
In most cases, these roundoff errors are not significant.
When they are significant, the problem itself is ill-conditioned and needs to be reformulated.

## What to do next

* Implement whole spec
* Document everything
* Implement `NonBlocking` mode
* Improve domain compatibility at the type level:
  * Add generic scalars on operations and ensure the types catch incompatible domains for each generic scalar. Current implementation narrows the different objects (vector, matrix) to same domain
* Iterate over masks implementation of `std::ops::Index` returning `false` for out-of-bounds indexes or if it should error out (prossibly new trait, which would lose on `value[index]` for the objects)
