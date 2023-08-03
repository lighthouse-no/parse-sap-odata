# Parse the Metadata from an SAP OData Service

This is a work in progress!

Parse the metadata XML describing an SAP OData service and generate Rust entities for each EDM type:

* [x] `ComplexType`
* [x] `EntityType`
* [ ] `FunctionImport`

> ***TODO***
>
> Currently when generating a Rust `struct`, only the `Name` and `Type` properties are extracted from the XML `<EntityType>` declaration.<br>
> Consider how the other XML attribute values and SAP annotations could be made available within the Rust `struct`.

---
## Usage

### Declare Build Dependency

In the `Cargo.toml` of your application, define an entry in `[build-dependencies]` that points to the `parse-sap-odata` crate:

```toml
[build-dependencies]
parse-sap-odata = "^1.1.0"
```

Your app will also require these dependencies

```toml
[dependencies]
rust_decimal = "^1.30"
uuid = "^1.4"
chrono = "^0.4"
```

### Create a Build Script

In your app's `build.rs`, run the generator for your desired OData service:

```rust
use parse_sap_odata::utils::parse_odata::gen_src;

fn main() {
    // gen_src() requires two arguments
    // 1) metadata_file_name: String  The name of the XML file in the ./odata directory
    //                                Do not include the '.xml' suffix in the file name!
    // 2) namespace:          String  The namespace defined in the <Schema> attribute of the OData XML
    gen_src("gwsample_basic", "GWSAMPLE_BASIC");
}
```

See the Rust documentation page for [build scripts](https://doc.rust-lang.org/cargo/reference/build-scripts.html) for more information.

### XML Input Files

All metadata XML for the OData services your app consumes must be located in the `./odata` directory immediately under your app's top level directory.

### Generated Output

If `cargo` detects a `build.rs` file in your project/crate, then it automatically populates the environment variable `OUT_DIR`.
This variable then points to the directory into which all build script output is written.

The default directory name is `target/debug/build/<your_package_name>/out`, and this is where you can find the generated `struct` declarations for the OData service.

You can specify your own value for `OUT_DIR` either by calling `cargo` with the `--out_dir` flag, or by creating your own `config.toml` file in the `./.cargo` directory.
See [Cargo Configuration](https://doc.rust-lang.org/cargo/reference/config.html) for more details.

---

## Referencing Generated Output

In the source code of your application, the generated OData `structs` can be referenced like this:

```rust
use chrono::Utc;

include!(concat!(env!("OUT_DIR"), "/gwsample_basic.rs"));

fn main() {
    let now = Utc::now().naive_utc();
    let bp = BusinessPartner {
        address: Address {
            address_type: Some(String::from("Dummay addres type")),
            building: Some(String::from("Dummy building")),
            city: Some(String::from("Dummy city")),
            country: Some(String::from("Dummy country")),
            postal_code: Some(String::from("Dummy postal code")),
            street: Some(String::from("Dummy street")),
        },
        business_partner_id: String::from("Dummy business partner id"),
        business_partner_role: String::from("Dummy business partner role"),
        changed_at: Some(now),
        company_name: String::from("Dummy company name"),
        created_at: Some(now),
        currency_code: String::from("GBP"),
        email_address: String::from("Dummy email address"),
        fax_number: Some(String::from("Dummy fax number")),
        legal_form: Some(String::from("Dummy legal form")),
        phone_number: Some(String::from("0123456789")),
        web_address: Some(String::from("Dummy website address")),
    };

    println!("{:#?}", bp);
}
```

---

## OData Complex Types

In the event an Entity Type definition uses a complex type, then the complex type is first created as a Rust `struct`.
The field in Rust `struct` that has this complex type is then defined using this `struct`.

An example of this is the `Address` property.

```xml
<EntityType Name="BusinessPartner" sap:content-version="1">
  <Key>
    <PropertyRef Name="BusinessPartnerID"/>
  </Key>
  <Property Name="Address" Type="GWSAMPLE_BASIC.CT_Address" Nullable="false"/>

  <!-- SNIP -->

</EntityType>
```

The Rust `struct` name is generated by trimming the namespace qualifier and (if present) the `CT_` prefix

```xml
<ComplexType Name="CT_Address">
  <Property Name="City" Type="Edm.String" MaxLength="40" sap:label="City" sap:semantics="city"/>
  <Property Name="PostalCode" Type="Edm.String" MaxLength="10" sap:label="Postal Code" sap:semantics="zip"/>
  <Property Name="Street" Type="Edm.String" MaxLength="60" sap:label="Street" sap:semantics="street"/>
  <Property Name="Building" Type="Edm.String" MaxLength="10" sap:label="Building"/>
  <Property Name="Country" Type="Edm.String" MaxLength="3" sap:label="Country" sap:semantics="country"/>
  <Property Name="AddressType" Type="Edm.String" MaxLength="2" sap:label="Address Type"/>
</ComplexType>
```

So the above XML definition becomes:

```rust
#[derive(Clone, Copy, Debug)]
pub struct Address {
    pub address_type: Option<String>,
    pub building: Option<String>,
    pub city: Option<String>,
    pub country: Option<String>,
    pub postal_code: Option<String>,
    pub street: Option<String>,
}
```

---

## OData "Simple" Complex Types

The metadata for the `GWSAMPLE_BASIC` OData service contains the following complex type:

```xml
<ComplexType Name="CT_String">
  <Property Name="String" Type="Edm.String" Nullable="false" sap:creatable="false" sap:updatable="false" sap:sortable="false" sap:filterable="false"/>
</ComplexType>
```

Allowing for the current situation in which additional attribute values and SAP Annotations are not preserved, this particular type turns out not to be complex at all &mdash; its just a `String`.
In such cases, fields declared to be of these "simple" complex types (such as `CT_String`), are collapsed down to the Rust native type of the single inner property &mdash; which in this example is simply a `String`.

---

## TODOs

1. Consider fetching the metadata at build time &mdash; but this raises the question of whether allowing a build script to look outside its sandbox is an anti-pattern...
1. Support Function Imports

---

## Testing this Crate Locally

1. Clone this repo
2. Change into the repo's `build_test_crate` subdirectory.
3. Run `cargo build`
4. Run `./target/debug/build-test-crate` and you will see output similar to this:

```rust
BusinessPartner {
    address: Address {
        address_type: Some(
            "Dummy address type",
        ),
        building: Some(
            "Dummy building",
        ),
        city: Some(
            "Dummy city",
        ),
        country: Some(
            "Dummy country",
        ),
        postal_code: Some(
            "Dummy postal code",
        ),
        street: Some(
            "Dummy street",
        ),
    },
    business_partner_id: "Dummy business partner id",
    business_partner_role: "Dummy business partner role",
    changed_at: Some(
        2023-08-03T10:42:57.532857,
    ),
    company_name: "Dummy company name",
    created_at: Some(
        2023-08-03T10:42:57.532857,
    ),
    currency_code: "GBP",
    email_address: "Dummy email address",
    fax_number: Some(
        "Dummy fax number",
    ),
    legal_form: Some(
        "Dummy legal form",
    ),
    phone_number: Some(
        "0123456789",
    ),
    web_address: Some(
        "Dummy website address",
    ),
}
```
