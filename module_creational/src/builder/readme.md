# Builder Pattern

Builder is a creational design pattern that lets you construct complex objects step by step. 
The pattern allows you to produce different types and representations of an object using the same construction code.

## Problem

Imagine a complex object that requires laborious, step-by-step initialization of many fields and nested objects. 
Such initialization code is usually buried inside a monstrous constructor with lots of parameters. 
Or even worse: scattered all over the client code.

## Solution

The Builder pattern suggests that you extract the object construction code out of its own class and move it to separate objects called builders.

## Applicability

* Use the Builder pattern to get rid of a “telescoping constructor”.
* Use the Builder pattern when you want your code to be able to create different representations of some product (for example, stone and wooden houses).
* Use the Builder to construct Composite trees or other complex objects.

## Pros and Cons

* ✅ Can construct objects step-by-step, defer construction steps or run steps recursively.
* ✅ Can reuse the same construction code when building various representations of products.
* ✅ Single Responsibility Principle. Can isolate complex construction code from the business logic of the product.
* ❎ The overall complexity of the code increases since the pattern requires creating multiple new classes.