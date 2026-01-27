import gsap from "gsap";
import { useEffect } from "react";
import "../fonts.css";

interface CardProps {
  width: string;
  height: string;
}

function Card(props: CardProps) {
  const w = "w-" + props.width;
  const h = "h-" + props.height;

  return (
    <div id="info-cards" className={`${w} ${h} rounded-2xl bg-black`}>
      <h1></h1>
    </div>
  );
}

export default function Welcome() {
  useEffect(() => {
    const timeline = gsap.timeline();

    timeline.to("#opening-text", {
      y: -130,
      opacity: 0,
      duration: 2,
      ease: "power3.out",
    });

    timeline.fromTo(
      "#info-cards",
      {
        y: 100,
        opacity: 0,
      },
      {
        
        opacity: 100,
      },
    );
  }, []);

  return (
    <section className="w-full h-screen flex flex-col justify-center items-center">
      <h1 id="opening-text" className="fixed top-1/2 text-2xl varela">
        Welcome to Depth34 Reflect
      </h1>

      <div className="grid grid-cols-4">
        <Card width={"50"} height={"50"} />
        <Card width={"50"} height={"50"} />
        <Card width={"50"} height={"50"} />
        <Card width={"50"} height={"50"} />
      </div>
    </section>
  );
}
