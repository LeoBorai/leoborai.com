---
title: "Uploading files to GraphQL Server with URQL"
description: "Quick guide on how to upload files to a GraphQL server using URQL and Svelte"
categories: [svelte, urql, graphql, file, upload]
icon: svelte
date: 2022-03-29
preview_image_url: "https://images.unsplash.com/photo-1544396821-4dd40b938ad3?ixlib=rb-1.2.1&ixid=MnwxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8&auto=format&fit=crop&w=2073&q=80"
published: true
---

Even though I have used GraphQL for a little while, I have never stepped upon a
file upload requirement. But theres always a first time no? And mine
was the last week, when I had to upload a image file as part of a GraphQL
`mutation`.

This is the schema in question:

```graphql
input BrandCreateInput {
  name: String!
  logo: Upload!
}

mutation ($input: BrandCreateInput!) {
  brandCreate(input: $input) {
    id
    name
    logo
  }
}
```

The `Upload` type above refers to [Apollo's GraphQL `Upload`][1] type.
The `Upload` type represents a `multipart/form-data` upload which is the one
used when uploading a file with the `FormData` object with your browser.

For my project I'm using [URQL][2], a GraphQL client by Formidable Labs which
brings bindings for React, Svelte, Vue and Vanilla JS. For the UI components
I'm using Svelte. For example purposes I will use the same stack. First we will
integrate URQL to our Svelte project, then we will define a service which will
be responsible of performing the `mutation` using the URQL client. And finally
we will implement a Svelte component which will have a form with a `name` text
input and a `logo` file input. Values from this form are then provided to the
service to perform the mutation.

## Setup URQL for your project

Let's install a couple packages:

- `urql`: The core library for URQL
- `@urql/svelte`: Svelte bindings for URQL
- `@urql/exchange-multipart-fetch`: A URQL [exchange][3] to enable file uploads
with `multipart/form-data`.

```bash
npm install urql @urql/svelte @urql/exchange-multipart-fetch
```

Then create a URQL client and provide the `multipartFetchExchange` from the
`@urql/exchange-multipart-fetch` library.

I'm exporting the instance of this URQL client from the module scope as follows:

```ts
// lib/utils/urql.ts

import { multipartFetchExchange } from '@urql/exchange-multipart-fetch';
import { createClient, dedupExchange, cacheExchange } from 'urql';

export const urqlClient = createClient({
  url: '/graphql',
  exchanges: [dedupExchange, cacheExchange, multipartFetchExchange],
});
```

## Defining a `ProductService` to be responsible for performing the `mutation`

I like to keep my logic and view or use case disengaged. so I tend to define
services which takes care of data manipulation and API consumption and then use
these services in my views or use cases.

The `ProductService` is responsible for performing product related operations,
for instance _creating new brands_.

```ts
// lib/services/product.ts

import { urqlClient } from '$lib/utils/urql';

import type { Brand } from '$graphql/schema';

const CreateBrandMutation = `
mutation ($input: BrandCreateInput!) {
  brandCreate(input: $input) {
    id
    name
    logo
  }
}`;

const createBrand = async (name: string, logo: File): Promise<Brand> => {
  const { data = {}, error } = await urqlClient
    .mutation(CreateBrandMutation, {
      input: {
        name,
        logo
      }
    })
    .toPromise();

  if (error) {
    // Do actual error handling here
    throw error;
  }

  return data.brandCreate;
}

export const productService = {
  createBrand,
}
```

## Retrieve the file from the user

With a URQL client in place, we are ready to actually ask the user for a file
to upload.

To handle form logic I'm using [manzana][4] ⏤ I'm the author ⏤ which provides
logic to handle form validation, values, errors and more with Svelte API
primitives such as stores.

> For example purposes, I will keep UI and logic as simple as possible.

```svelte
<!-- CreateBrand.svelte -->

<script lang="ts">
  import { newForm } from 'manzana';

  import { productService } from '$lib/services/product';

  const { handleSubmit, values, setFieldValue } = newForm<{
    name: string;
    logo: File;
  }>({
    initialValues: {
      name: '',
      logo: null
    },
    onSubmit: async (values) => {
      await productService.createBrand(values.name, values.logo);
    }
  });
</script>

<form on:submit={handleSubmit}>
  <input
    type="text"
    name="name"
    placeholder="Apple Inc."
    bind:value={$values.name}
  />
  <input
    type="file"
    on:change={(event) => {
      const files: FileList = event.target.files;

      setFieldValue('logo', files[0]);
    }}
  />
  <button type="submit">
    Create
  </button>
</form>
```

## Conclusion

Voilà! We have a file upload to a GraphQL server using Svelte and URQL!
If you had any issues following up, please open a PR or issue [here][5]. I'm
always happy to learn and help others!

[1]: https://www.apollographql.com/blog/graphql/file-uploads/with-apollo-server-2-0/#File-upload-with-schema-param
[2]: https://formidable.com/open-source/urql/
[3]: https://formidable.com/open-source/urql/docs/architecture/#the-client-and-exchanges
[4]: https://github.com/LeoBorai/manzana
[5]: https://github.com/LeoBorai/leoborai.com
