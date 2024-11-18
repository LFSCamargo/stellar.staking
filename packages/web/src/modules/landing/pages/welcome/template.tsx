type WelcomeTemplateProps = {
  name: string;
};

function WelcomeTemplate({ name }: WelcomeTemplateProps) {
  return (
    <div className="flex h-screen flex-col items-center justify-center">
      <h1 className="text-3xl font-bold text-black" aria-label="Hello World">
        Welcome, {name}
      </h1>
      <p className="text-xl font-bold text-neutral-700">Life is good, you know what I mean?</p>
    </div>
  );
}

export default WelcomeTemplate;
