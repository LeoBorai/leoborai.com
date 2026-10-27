---
title: "Pxid: Developer Friendly IDs"
description: "When it comes to work with taxonomy rich APIs, having descriptive IDs that provides context about the resource in question is crucial"
categories: [id, api, graphql, design, software, rust]
icon: rust
date: 2023-07-22
preview_image_url: "https://images.unsplash.com/photo-1544383835-bda2bc66a55d?ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D&auto=format&fit=crop&w=2272&q=1"
published: true
---

## Inspiration

Back in December 2022 I met the Stripe API, the experience is incredible it
feels like the developers behind the API really think about the Developer
Experience.

They provide you with a test environment for your integrations so you can work
without worring about cleanning up the environment when you move into production,
they provide monitoring tools with an inspector for all the requests to the API
so you can easily investigate on errors and payloads related to them, but one
of the things that I liked the most was their resource IDs.

Stripe's resource IDs are insigthful, they are prefixed with the resource name
ID and they are also short, these features make Stripe's IDs easy to remember
and also easy to work with.

```bash
cus_9s6XKzkNRiz8i3 # A Customer ID from https://stripe.com/docs/api/customers
file_1LzR9L2eZvKYlo2CelTpcvKu # A file ID from https://stripe.com/docs/api/files
```

### What about the `object` key in the JSON Payload?

If you visit the Stripe API documentation or you just already worked with it,
you will notice that the JSON objects comes with an `object` field which value
is the type of object represented by the response fragment.

```js
{
  "id": "file_1LzR9L2eZvKYlo2CelTpcvKu",
  "object": "file",
  "created": 1667334811,
  "expires_at": null,
  "filename": "c2c-logo.png",
  "links": {
    "object": "list",
    "data": [
      {
        "id": "link_1NWqFH2eZvKYlo2CjegislO8",
        "object": "file_link",
        "created": 1690072439,
        "expired": false,
        "expires_at": null,
        "file": "file_1LzR9L2eZvKYlo2CelTpcvKu",
        "livemode": false,
        "metadata": {},
        "url": "https://files.stripe.com/links/MDB8YWNjdF8xMDMyRDgyZVp2S1lsbzJDfGZsX3Rlc3RfVzVXajVISGh0UEYwSTdMdno2Zjd5OTQx006747U4IB"
      },
// -- snip --
// Source: https://stripe.com/docs/api/files/object
```

The thing is that you may not want to include this value within your response
object, and if you are using GraphQL either the playground or using reflection
will give you enough context to identify the object in question.

### Looking for an implementation

So I started looking for a similar implementation, I wanted to have IDs which
are:

- Cheap to Generate
- Unique
- Sortable

So I started to investigate and found some ID designs out there, including:

| Name | Bytes | Length | Configuration Free | Sortable | Dependency Free (*) |
| --- | --- | --- | --- | --- | --- |
| ObjectID | 12 | 24 | ✅ | ✅ | ✅ |
| UUID | 16 | 36 | ✅ | ❎ | ✅ |
| Snowflaxe | 8 | ≈20 | ❎ | ✅ | ❎ |
| xid | 12 | 20 | ✅ | ✅ | ✅ |

> (*) System Time is not being considered dependency in this table

Most of these IDs are or too long or expensive to generate or requires
external dependencies, but `xid` sounded very promissing to me, so I moved into
that direction.

## Getting to know XID

Xid is implemented by Oliver Poitrey who is Director of Engineering at Netflix
Co-Founder & ex-CTO of Dailymotion, the original implementation is done on
Golang.

The ID generation is pretty fast. [Copy of the snippet from the Docs][1].

```
BenchmarkXID        	20000000	       91.1 ns/op	      32 B/op	       1 allocs/op
BenchmarkXID-2      	20000000	       55.9 ns/op	      32 B/op	       1 allocs/op
BenchmarkXID-4      	50000000	       32.3 ns/op	      32 B/op	       1 allocs/op
BenchmarkUUIDv1     	10000000	       204 ns/op	      48 B/op	       1 allocs/op
BenchmarkUUIDv1-2   	10000000	       160 ns/op	      48 B/op	       1 allocs/op
BenchmarkUUIDv1-4   	10000000	       195 ns/op	      48 B/op	       1 allocs/op
BenchmarkUUIDv4     	 1000000	      1503 ns/op	      64 B/op	       2 allocs/op
BenchmarkUUIDv4-2   	 1000000	      1427 ns/op	      64 B/op	       2 allocs/op
BenchmarkUUIDv4-4   	 1000000	      1452 ns/op	      64 B/op	       2 allocs/op
```

### Anatomy of the ID

An instance of XID is made of 4 resources: MachineID, Process ID, Current Time
and an Atomic Counter.

Generating an instance in Golang looks like this

```go
// Snippet from: https://github.com/rs/xid/tree/aa67b0c86a8afa78f6fca7a3111d33fef696f90f#usage

guid := xid.New()

println(guid.String())
// Output: 9m4e2mr0ui3e8a215n4g
```

### Cryptographical Security and Unpredictability

As every software-related resource, theres no "Golden Bullets" in this area,
even though XIDs sounds promissing theres a drawback, given to its dependencies
an instance of XID is not cryptographically secure.

This is because one could attemp to generate an ID based on related resources,
as stated in the README:

```
Xid is dependent on the system time, a monotonic counter and so is not
cryptographically secure. If unpredictability of IDs is important, you should
not use Xids.

It is worth noting that most other UUID-like implementations are also not
cryptographically secure. You should use libraries that rely on
cryptographically secure sources (like /dev/urandom on unix,
crypto/rand in golang), if you want a truly random ID generator.
```

## Meet Pxid

So after reading on the [XID README][2] and checking on its source code I decided
that XIDs are very good for my purpose and started extending the existing
implementation but in Rust, after writing a prefixed version of Xid I ended up
with Prefixed Globally Unique Identifier (`Pxid`).

```
 V V V V W W W W X X X Y Y Z Z Z
 └─────┘ └─────┘ └───┘ └─┘ └───┘
    |       |      |    |    |
 Prefix  Timestamp |   PID   |
                   |       Counter
                   |
               Machine ID
          └────────────────────┘
                   |
                  XID
```

With a total of 16 bytes I can store an easy to read, cheap to generate and
human-friendly ID.

The prefix is a 4 byte UTF-8 compliant string, followed by a downscore (`_`) and
the Xid.

```txt
acct_9m4e2mr0ui3e8a215n4g
ordr_9m4e2mr0ui3e8a215n4g
usr_9m4e2mr0ui3e8a215n4g
```

### Usage

You can install Pxid in your project using `cargo add pxid`, then generate IDs
as follows:

```rust
use pxid::Pxid;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Given that some of the dependencies to build
    // an instance of the Pxid may fail.
    // - Getting current timestamp
    // - Getting machine id
    // - Getting process id
    //
    // A `Result<Pxid, Error>` is returned.
    let id = Pxid::new("acct".as_bytes())?;

    println!("{}", id); // acct_9m4e2mr0ui3e8a215n4g
}
```

By using `Pxid::new` you will be allocating many resources, if you want to
generate multiple instances I recommend using the `Factory` approach:

```rust
use pxid::Factory;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let factory = Factory::new_without_prefix()?;
    let id = factory.with_prefix("acct");

    println!("{}", id); // acct_9m4e2mr0ui3e8a215n4g

    let factory_with_prefix = Factory::new("acct")?;
    let id = factory_with_prefix.generate();

    println!("{}", id); // acct_9m4e2mr0ui3e8a215n4g
}
```

## Conclusion

Theres no golden bullet for this field as stated earlier, but of course you have
many options out there! Feel free to contribute to Xid or Pxid anytime on GitHub!

Have a happy hacking!

[1]: https://github.com/rs/xid/tree/aa67b0c86a8afa78f6fca7a3111d33fef696f90f#benchmark
[2]: https://github.com/rs/xid/blob/aa67b0c86a8afa78f6fca7a3111d33fef696f90f/README.md
