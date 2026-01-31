import gsap from "gsap";
import { ExpoScaleEase } from "gsap/EasePack";
import { useEffect, useRef, useState } from "react";
import { ArrowUp } from "lucide-react";
import "./fonts.css";

interface CardProps {
  title: string;
  className?: string;
  dropdown?: boolean;
  children?: React.ReactNode;
}

function Card({ title, className, dropdown, children }: CardProps) {
  const [open, setOpen] = useState(false);

  return (
    <div
      className={`info-card ${className} rounded-2xl bg-white/10 backdrop-blur-xl border border-white/10 shadow-xl p-4 flex flex-col snap-center min-w-[85vw] md:min-w-0`}
    >
      <div className="flex justify-between items-center">
        <h2 className="text-white text-lg varela">{title}</h2>
        {dropdown && (
          <button
            onClick={() => setOpen(!open)}
            className="text-blue-300 text-sm"
          >
            <ArrowUp
              className={`${open ? "rotate-0" : "rotate-180"} duration-200 cursor-pointer`}
            />
          </button>
        )}
      </div>

      <div
        className={`mt-3 text-gray-200 text-sm transition-all duration-300 overflow-hidden ${dropdown ? (open ? "max-h-96 opacity-100" : "max-h-0 opacity-0") : ""}`}
      >
        {children || ""}
      </div>

      {!dropdown && (
        <div className="mt-3 text-gray-200 text-sm">{children || ""}</div>
      )}
    </div>
  );
}

export default function Welcome() {
  const containerRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    gsap.registerPlugin(ExpoScaleEase);

    const tl = gsap.timeline();

    tl.to("#opening-text", {
      y: -120,
      opacity: 0,
      duration: 1.8,
      ease: "expoScale(0.5,7,none)",
    });

    tl.fromTo(
      ".info-card",
      { y: 40, opacity: 0, scale: 0.9 },
      {
        y: 0,
        opacity: 1,
        scale: 1,
        stagger: 0.08,
        duration: 0.8,
        ease: "power3.out",
      },
      "-=0.5",
    );
  }, []);

  return (
    <section className="w-full h-screen bg-[#020617] flex justify-center items-center relative overflow-hidden">
      <div className="fixed bg-blue-500/40 -top-32 -left-32 w-96 h-96 rounded-full blur-[120px]" />
      <div className="fixed bg-emerald-500/40 -bottom-32 -right-32 w-96 h-96 rounded-full blur-[120px]" />

      <h1 id="opening-text" className="absolute text-3xl text-white varela">
        Depth34 Reflect
      </h1>

      <div
        ref={containerRef}
        style={{ scrollbarWidth: "none", msOverflowStyle: "none" }}
        className="w-full h-[80%] md:w-[90%] flex md:grid md:grid-cols-6 md:grid-rows-4 gap-5 overflow-x-auto md:overflow-hidden snap-x snap-mandatory px-5 md:px-0 [&::-webkit-scrollbar]:hidden"
      >
        <Card title="Title" className="md:col-span-4 md:row-span-2"></Card>

        <Card title="Title" className="md:col-span-2 md:row-span-1"></Card>

        <Card title="Title" className="md:col-span-1 md:row-span-1"></Card>

        <Card title="Title" className="md:col-span-1 md:row-span-1"></Card>

        <Card
          title="Title"
          className="md:col-span-2 md:row-span-2"
          dropdown
        ></Card>

        <Card title="Title" className="md:col-span-4 md:row-span-2"></Card>
      </div>
    </section>
  );
}
