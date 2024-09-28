import type { Band, Manager, Musician, User } from "~/types";

export const defaultUser: User = {
  id: '',
  name: '',
  lastname: '',
  email: '',
  phone: '',
  phone_prefix: '',
  country: '',
  city: '',
  street: '',
  house_number: '',
  apartment: null,
  types: [],
  description: '',
  password: '',
};


export const defaultBand: Band = {
  name: '',
  description: '',
  country_of_origin: '',
  established_in: null as unknown as number,
  instruments: [],
  links: [],
  managers: [],
  members: [],
  music_styles: [],
  searching_for: [],
};

export const defaultMusician: Musician = {
  user: defaultUser,
  stage_name: '',
  bands: [],
  skills: [],
  links: [],
  managers: [],
  open_to_collab_with: [],
};

export const defaultManager: Manager = {
  user: defaultUser,
  stage_name: '',
  commission: [null as unknown as number, null as unknown as number],
  bands: [],
  categories_interested_in: [],
};