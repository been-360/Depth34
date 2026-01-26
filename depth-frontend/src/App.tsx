import gsap from "gsap";
import { useEffect } from "react";
import "./fonts.css";

interface CardProps {
  width: string;
  height: string;
}

function Card(props: CardProps) {
  const w = "w-" + props.width;
  const h = "h-" + props.height;

  return (
    <div id="info-cards" className={`${w} ${h} rounded-2xl bg-gray-100`}>
      <h1></h1>
    </div>
  );
}

export default function Welcome() {
  useEffect(() => {
    const timeline = gsap.timeline();

    timeline.to("#opening-text", {
      y: -140,
      opacity: 0,
      duration: 3,
     ease: "expoScale(0.5,7,none)",
  
    });

    timeline.fromTo(
      "#info-cards",
      { opacity: 0 },
      {
        y: -140,
        opacity: 100,
        duration: 3,
        ease: "bounce.in",
      },
    );
  }, []);

  return (
    <section className="w-full h-screen flex flex-col justify-center items-center">
      <div className="fixed bg-blue-300 -top-20 -left-20 w-100 h-150 rounded-full blur-[990px]" />
      <div className="fixed bg-emerald-500 -bottom-10 -right-10 w-70 h-70 rounded-full blur-[990px]" />

      <h1 id="opening-text" className="fixed top-1/2 text-2xl varela">
        Welcome to Depth34 Reflect
      </h1>

      <div className="grid grid-cols-4">
      </div>
    </section>
  );
}
