import type { Band, Manager, Musician, User } from "~/types";

export const defaultUser: User = {
  name: '',
  lastname: '',
  email: '',
  phone: '',
  phonePrefix: '',
  country: '',
  city: '',
  street: '',
  houseNumber: '',
  apartment: undefined,
  description: '',
  password: '',
};


export const defaultBand: Band = {
  name: '',
  description: '',
  countryOfOrigin: '',
  establishedIn: null as unknown as number,
  instruments: [],
  links: [],
  managers: [],
  members: [],
  musicStyles: [],
  searchingFor: [],
};

export const defaultMusician: Musician = {
  ...defaultUser,
  stageName: '',
  bands: [],
  skills: [],
  description: '',
  links: [],
  managers: [],
  openToCollabWith: [],
};

export const defaultManager: Manager = {
  ...defaultUser,
  stageName: '',
  commission: [null as unknown as number, null as unknown as number],
  bands: [],
  categoriesInterestedIn: [],
};