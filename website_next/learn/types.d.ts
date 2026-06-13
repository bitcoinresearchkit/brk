declare global {
  type LearnSection = {
    title: string;
    description: string;
    chart?: Chart;
    numbered?: boolean;
    children?: LearnSection[];
  };

  type LearnDetails = {
    openHash(hash: string): void;
    toggleHash(hash: string): boolean;
  };
}

export {};
