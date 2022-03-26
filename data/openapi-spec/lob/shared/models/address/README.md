# Lob Address Model(s)

Abandon hope all ye who enter here...

Seriously, though, addresses are complicated.

## Why are there so many models?

Because:

- behavior varies by whether an address is US or international
- we require ISO 2 letter country codes on input, but store and display the ISO full country names
- OpenAPI v3.0 has kinda clunky polymorphism support

## Sigh. Ok, what is all this stuff?

1. Input models:

   - `address_editable_*.yml` - Properties of the address model that customers can specify.
   - `address_fields_*.yml` - These properties can also be specified, but are also in the stored model unchanged.
   - `address_editable.yml` - The polymorphic address creation model. The discriminator field tells the API which variant to use. In OpenAPI v3.0, all possible discriminator values must be listed.
   - `inline_address*.yml` - Inline mappings of the previous models, used by the form factors.

1. Stored models:

   - `address_us.yml` and `address_intl.yml` - Like the editable versions, these are used in `address_country` with the discriminator. In order for polymorphism to work for the stored version of address, the version of `address_country` used by `address_editable.yml` must be completely hidden, hence the plethora of models.
   - `address.yml` - The polymorphic stored address model. As before, the discriminator field specifies the variant, except this time the discriminator field, despite having the same name, contains the full name of the country.
