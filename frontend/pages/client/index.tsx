import { MinimalMenu } from "@/components/minimal_menu";
import { FormPage, FormState } from "@/components";
import { CLIENT_MENU_ICON } from "@/public/client-menu-icon";

interface AssortmentType {
  assortment: Array<string>
}

export async function getServerSideProps(context) {
  const assortment_response = await fetch("http://127.0.0.1:42173/assortment", {
    method: "GET"
  });

  const return_val: AssortmentType = await assortment_response.json();

  console.log("RESPONSE " + return_val.assortment);
  return {
    props: {
      server_data: return_val
    }
  };
}

export default function Client({ server_data }: { server_data: AssortmentType }) {
  //const client_options: Array<[string, string]> = [];
  //??client_options.push(["Informacje ogólne", "/client"]);
  //client_options.push()
  const client_options: Array<[string, string]> = [["Informacje ogólne", "/client"], ["Wyszukaj klienta", "/client/search"], ["Dodaj klienta", "/client/add"]];
  let state = new FormState(server_data.assortment);

  return (
    <>
      <MinimalMenu image_svg={CLIENT_MENU_ICON} menu_options={client_options} />
      <div className="m-15 flex grow items-center justify-center">
        <FormPage state={state} />
      </div>
    </>
  );
}
