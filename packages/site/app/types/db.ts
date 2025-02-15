export type YearFromTo<T> = {
  from: string,
  to: string,
  item: T,
};

export type User = {
  name: string;
  lastname: string;
  email: string;
  phone: string;
  phonePrefix: string;
  country: string;
  city: string;
  street: string;
  houseNumber: string;
  apartment: string | undefined;
  password: string;
  description: string;
};

export type Band = {
  name: string;
  establishedIn: number;
  description: string;
  countryOfOrigin: string;
  members: YearFromTo<Musician & { mainPurpose: string }>[];
  musicStyles: string[];
  instruments: string[];
  links: string[];
  managers: YearFromTo<Manager>[] | undefined;
  // TODO: possibly an enum(?)
  searchingFor: string[];
};

export type Musician = User & {
  stageName: string;
  bands: YearFromTo<Band>[];
  skills: {
    from: string;
    to: string;
    level: number;
    name: string;
  }[];
  description: string;
  links: string[];
  managers: YearFromTo<Manager>[] | undefined;
  // TODO: possibly an enum (?)
  openToCollabWith: string[];
};

export type Manager = User & {
  stageName: string;
  commission: [number,number];
  bands: YearFromTo<Band>[];
  categoriesInterestedIn: string[];
};