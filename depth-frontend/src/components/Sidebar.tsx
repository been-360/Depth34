import { useState } from "react";
import { Menu } from "lucide-react";

export default function Sidebar() {
  const [isOpen, setIsOpen] = useState(false);

  return (
    <div
      className={`justify-center flex ${isOpen ? "w-70" : "w-15"} h-full bg-black duration-200`}
    >
      <Menu
        color="white"
        className={"cursor-pointer relative top-4"}
        onClick={() => setIsOpen((prev) => !prev)}
      />
    </div>
  );
}
