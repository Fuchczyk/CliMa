import Image from "next/image";
import Link from "next/link";
import { Url } from "next/dist/shared/lib/router/router";
import { FormEvent } from "react";
import { redirect } from "next/navigation";
import { useRouter } from "next/router";

class MenuEntry {
  name: String;
  href: Url;

  constructor(name: String, href: Url) {
    this.name = name;
    this.href = href;
  }
}

export function Navbar() {
  const bar_options = [];
  bar_options.push(new MenuEntry("Strona główna", "/"));
  bar_options.push(new MenuEntry("Klient", "/client"));
  bar_options.push(new MenuEntry("Rozmowa", "/call"));
  bar_options.push(new MenuEntry("Towar", "/cargo"));


  const navbar_items = bar_options.map((entry) =>
    <Link href={entry.href} className="hover:text-button hover:underline">{entry.name}</Link>
  );

  return (
    <nav className="flex items-center bg-navigation">
      <div className="flex items-center p-2">
        <Image src="/phone-booth.svg" alt="Phone booth" className="h-11" width={60} height={60} />
        <span className="self-center whitespace-nowrap pl-1 text-2xl font-bold text-white">CliMa</span>
      </div>
      <div className="ml-10 flex grow flex-row items-center justify-end space-x-9 pr-5 text-white">
        {navbar_items}
      </div>
    </nav>
  );
}

export class FormState {
  name: string;
  email: string;
  postcode: string;
  city: string;
  phone: string;
  assortment: Map<string, boolean>;
  id: number | null;

  constructor(assortment: Array<string>) {
    this.name = "";
    this.email = "";
    this.postcode = "";
    this.city = "";
    this.phone = "";
    this.id = null;

    this.assortment = new Map();

    for (const assort_name of assortment) {
      this.assortment.set(assort_name, false);
    }
  }
}

export function FormPage({ state }: { state: FormState }) {
  const router = useRouter();

  const handle_submit = async (event: FormEvent<HTMLFormElement>) => {
    event.preventDefault();
    const data = new FormData(event.currentTarget);

    let assort_preferences = [];
    console.log(state);

    for (const [key, _] of data.entries()) {
      if (state.assortment.has(key)) {
        assort_preferences.push(key);
      }
    }

    const form_data = {
      name: data.get('name'),
      email: data.get('email'),
      postcode: data.get('postcode'),
      city: data.get('city'),
      phone: data.get('phone'),
      assortment: assort_preferences,
      id: state.id
    };

    console.log("BODY: " + JSON.stringify(form_data));

    try {
      const response = await fetch("http://127.0.0.1:42173/client", {
        method: "POST",
        body: JSON.stringify(form_data),
        headers: {
          "Content-type": "application/json; charset=UTF-8"
        }
      });

      const json_body = await response.json();
      console.log(json_body);
    }
    catch (e) {
      console.log("ERROR" + e);
    }
  };

  const checkbox_menu = Array.from(state.assortment, ([key, value]) => (
    <div className="col-span-1 mb-4 flex items-center">
      <input className="peer h-4 w-4 rounded border-background bg-gray-100" id={key} name={key} type="checkbox" defaultChecked={value} />
      <label htmlFor={key} className="peer-checked:text-yellow-theme ml-2 text-sm font-medium text-background peer-checked:font-semibold">
        {key}
      </label>
    </div>
  ));

  return (
    <form className="border-8 border-double border-background bg-navigation p-6" onSubmit={handle_submit}>
      <div className="group relative z-0 mb-6 w-full">
        <input
          type="text"
          name="name"
          id="name"
          className="peer block w-full appearance-none border-0 border-b-2 border-background bg-transparent py-2.5 px-0 text-sm text-white focus:border-picture focus:outline-none focus:ring-0"
          defaultValue={state.name}
          placeholder=" "
          required={true} />
        <label
          htmlFor="name"
          className="absolute top-3 -z-10 origin-[0] -translate-y-6 scale-75 transform text-sm text-background duration-300 peer-placeholder-shown:translate-y-0 peer-placeholder-shown:scale-100 peer-focus:left-0 peer-focus:-translate-y-6 peer-focus:scale-75 peer-focus:font-medium peer-focus:text-picture"
        >Nazwa klienta</label>
      </div>

      <div className="group relative z-0 mb-6 w-full">
        <input
          type="text"
          name="email"
          id="email"
          className="peer block w-full appearance-none border-0 border-b-2 border-background bg-transparent py-2.5 px-0 text-sm text-white focus:border-picture focus:outline-none focus:ring-0"
          defaultValue={state.email}
          placeholder=" "
          required={true} />
        <label
          htmlFor="name"
          className="absolute top-3 -z-10 origin-[0] -translate-y-6 scale-75 transform text-sm text-background duration-300 peer-placeholder-shown:translate-y-0 peer-placeholder-shown:scale-100 peer-focus:left-0 peer-focus:-translate-y-6 peer-focus:scale-75 peer-focus:font-medium peer-focus:text-picture"
        >E-mail</label>
      </div>

      <div className="grid md:grid-cols-3 md:gap-6">
        <div className="group relative z-0 mb-6 w-full">
          <input
            type="text"
            name="postcode"
            id="postcode"
            defaultValue={state.postcode}
            className="peer block w-full appearance-none border-0 border-b-2 border-background bg-transparent py-2.5 px-0 text-sm text-white focus:border-picture focus:outline-none focus:ring-0"
            placeholder=" "
            required={true}
          />
          <label
            htmlFor="postcode"
            className="absolute top-3 -z-10 origin-[0] -translate-y-6 scale-75 transform text-sm text-background duration-300 peer-placeholder-shown:translate-y-0 peer-placeholder-shown:scale-100 peer-focus:left-0 peer-focus:-translate-y-6 peer-focus:scale-75 peer-focus:font-medium peer-focus:text-picture"
          >Kod pocztowy</label>
        </div>
        <div className="group col-span-2 relative z-0 mb-6 w-full">
          <input
            type="text"
            name="city"
            id="city"
            defaultValue={state.city}
            className="peer block w-full appearance-none border-0 border-b-2 border-background bg-transparent py-2.5 px-0 text-sm text-white focus:border-picture focus:outline-none focus:ring-0"
            placeholder=" "
            required={true}
          />
          <label
            htmlFor="city"
            className="absolute top-3 -z-10 origin-[0] -translate-y-6 scale-75 transform text-sm text-background duration-300 peer-placeholder-shown:translate-y-0 peer-placeholder-shown:scale-100 peer-focus:left-0 peer-focus:-translate-y-6 peer-focus:scale-75 peer-focus:font-medium peer-focus:text-picture"
          >Miasto</label>
        </div>
      </div>

      <div className="group relative z-0 mb-6 w-full">
        <input
          type="text"
          name="phone"
          id="phone"
          defaultValue={state.phone}
          placeholder=" "
          className="peer block w-full appearance-none border-0 border-b-2 border-background bg-transparent py-2.5 px-0 text-sm text-white focus:border-picture focus:outline-none focus:ring-0"
          required={true}
        />
        <label
          htmlFor="phone"
          className="absolute top-3 -z-10 origin-[0] -translate-y-6 scale-75 transform text-sm text-background duration-300 peer-placeholder-shown:translate-y-0 peer-placeholder-shown:scale-100 peer-focus:left-0 peer-focus:-translate-y-6 peer-focus:scale-75 peer-focus:font-medium peer-focus:text-picture"
        >Numer telefonu</label>
      </div>

      <div className="grid grid-cols-3">
        {checkbox_menu}
      </div>

      <input type="submit" value="Submit" />

    </form>
  );
}
