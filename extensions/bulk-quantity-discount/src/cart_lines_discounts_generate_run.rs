use super::schema;
use shopify_function::prelude::*;
use shopify_function::Result;
#[shopify_function]
fn cart_lines_discounts_generate_run(input:schema::cart_lines_discounts_generate_run::Input)->Result<schema::CartLinesDiscountsGenerateRunResult>{
    if !input.discount().discount_classes().contains(&schema::DiscountClass::Product){return Ok(schema::CartLinesDiscountsGenerateRunResult{operations:vec![]});}
    let mut candidates=vec![];
    for line in input.cart().lines().iter(){
        let variant=match line.merchandise(){schema::cart_lines_discounts_generate_run::input::cart::lines::Merchandise::ProductVariant(variant)=>variant,_=>continue};
        let quantity=*line.quantity();
        if !variant.product().has_bulk_discount_tag()||quantity<11{continue;}
        let (percentage,message)=if quantity>=50{(30.0,"30% off when buying 50+")}else{(20.0,"20% off when buying 11+")};
        candidates.push(schema::ProductDiscountCandidate{targets:vec![schema::ProductDiscountCandidateTarget::CartLine(schema::CartLineTarget{id:line.id().clone(),quantity:None})],message:Some(message.to_string()),value:schema::ProductDiscountCandidateValue::Percentage(schema::Percentage{value:Decimal(percentage)}),associated_discount_code:None});
    }
    if candidates.is_empty(){return Ok(schema::CartLinesDiscountsGenerateRunResult{operations:vec![]});}
    Ok(schema::CartLinesDiscountsGenerateRunResult{operations:vec![schema::CartOperation::ProductDiscountsAdd(schema::ProductDiscountsAddOperation{candidates,selection_strategy:schema::ProductDiscountSelectionStrategy::All})]})
}
#[cfg(test)]
mod tests {
    use super::*;
    use shopify_function::run_function_with_input;
    fn input(quantity:i32,tagged:bool)->String{format!(r#"{{"cart":{{"lines":[{{"id":"gid://shopify/CartLine/1","quantity":{},"merchandise":{{"__typename":"ProductVariant","product":{{"hasBulkDiscountTag":{}}}}}}}]}},"discount":{{"discountClasses":["PRODUCT"]}}}}"#,quantity,tagged)}
    fn percentage(quantity:i32,tagged:bool)->Result<Option<f64>>{let result=run_function_with_input(cart_lines_discounts_generate_run,&input(quantity,tagged))?;let operation=match result.operations.first(){Some(schema::CartOperation::ProductDiscountsAdd(operation))=>operation,_=>return Ok(None)};let value=match &operation.candidates[0].value{schema::ProductDiscountCandidateValue::Percentage(value)=>value.value.0,_=>return Ok(None)};Ok(Some(value))}
    fn message(quantity:i32)->Result<Option<String>>{let result=run_function_with_input(cart_lines_discounts_generate_run,&input(quantity,true))?;let operation=match result.operations.first(){Some(schema::CartOperation::ProductDiscountsAdd(operation))=>operation,_=>return Ok(None)};Ok(operation.candidates[0].message.clone())}
    #[test]
    fn regular_price_through_ten()->Result<()>{assert_eq!(percentage(10,true)?,None);Ok(())}
    #[test]
    fn twenty_percent_from_eleven_through_forty_nine()->Result<()>{assert_eq!(percentage(11,true)?,Some(20.0));assert_eq!(percentage(49,true)?,Some(20.0));Ok(())}
    #[test]
    fn thirty_percent_at_fifty_and_above()->Result<()>{assert_eq!(percentage(50,true)?,Some(30.0));assert_eq!(percentage(200,true)?,Some(30.0));Ok(())}
    #[test]
    fn tier_messages_explain_the_thresholds()->Result<()>{assert_eq!(message(11)?,Some("20% off when buying 11+".to_string()));assert_eq!(message(49)?,Some("20% off when buying 11+".to_string()));assert_eq!(message(50)?,Some("30% off when buying 50+".to_string()));Ok(())}
    #[test]
    fn untagged_products_are_ignored()->Result<()>{assert_eq!(percentage(200,false)?,None);Ok(())}
}
