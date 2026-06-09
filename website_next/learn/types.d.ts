declare global {
  type LearnSection = {
    title: string;
    description: string;
    chart?: Chart;
    numbered?: boolean;
    children?: LearnSection[];
  };
}

export {};
