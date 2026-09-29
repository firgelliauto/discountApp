import type { LoginError } from "@shopify/shopify-app-react-router/server";
import { LoginErrorType } from "@shopify/shopify-app-react-router/server";
export function loginErrorMessage(errors:LoginError):{shop?:string}{if(errors?.shop===LoginErrorType.MissingShop)return {shop:"Please enter your shop domain"};if(errors?.shop===LoginErrorType.InvalidShop)return {shop:"Please enter a valid shop domain"};return {};}
