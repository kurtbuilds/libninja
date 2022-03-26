# Form Factor Models

These models are common to all form factors.

## Field Guide

1. Input models

   - `editable.yml` - Properties which customers can customize
   - `editable_no_mailtype.yml` - A special case of `editable` for form factors that need different information w/re mailtype.
   - `input_to.yml` - `to`, which has rules sufficiently complex that it is its own model. Thus far, `to` is the same across Form Factors.
   - `input_from.yml` - `from` has different rules sufficiently complex that it is own model.
   - `input_from_us.yml` - Most Form Factors only allow `from` addresses in the US, so we have two variations of `from`.

1. stored models
   - `generated.yml` - Properties which we generate that are common to all form factors.
   - `from.yml` - The stored version of `from`. `to` is the same across all form factors, so is included in `generated.yml`.
   - `from_us.yml` - For form factors that only allow mailpieces to be sent from the United States.
   - `lob_base.yml` (in the parent directory) - Properties we generate that are common to all Print and Mail resources.
