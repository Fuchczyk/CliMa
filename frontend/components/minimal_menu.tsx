import Link from "next/link";


export function MinimalMenu({ image_svg, menu_options }: { image_svg: string; menu_options: Array<[string, string]> }) {
  const rendered_menu = menu_options.map(([name, path]) => (
    <div className="hover:border-green-theme hover:text-green-theme border-background mx-4 my-4 rounded border text-center">
      <Link href={path} className="m-5">{name}</Link>
    </div>
  ));

  return (
    <div className="text-background flex border-spacing-2 flex-col items-center pt-5 pl-5">
      <div className="bg-navigation rounded-md">
        <>
          <div className="stroke-picture fill-picture" dangerouslySetInnerHTML={{ __html: image_svg }}>
          </div>
          {rendered_menu}
        </>
      </div>
    </div>
  );
}
