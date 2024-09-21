import type { ModuleOptions } from "@nuxtjs/tailwindcss";
import plugin from 'tailwindcss/plugin';

export default {
  prefix: "",
  
  theme: {
    container: {
      center: true,
      padding: "2rem",
      screens: {
        "2xl": "1400px",
      },
    },
    extend: {
      colors: {
        border: "rgb(var(--border))",
        input: "rgb(var(--input))",
        ring: "rgb(var(--ring))",
        background: "rgb(var(--background))",
        foreground: "rgb(var(--foreground))",
        rhino: "rgb(var(--rhino))",
        chestnut: "rgb(var(--chestnut))",
        supernova: "rgb(var(--supernova))",
        greyAccent: "rgb(var(--grey-accent))",
        primary: {
          DEFAULT: "rgb(var(--primary))",
          foreground: "hsl(var(--primary-foreground))",
        },
        secondary: {
          DEFAULT: "rgb(var(--secondary))",
          foreground: "hsl(var(--secondary-foreground))",
        },
        destructive: {
          DEFAULT: "hsl(var(--destructive))",
          foreground: "hsl(var(--destructive-foreground))",
        },
        muted: {
          DEFAULT: "hsl(var(--muted))",
          foreground: "hsl(var(--muted-foreground))",
        },
        accent: {
          DEFAULT: "rgb(var(--accent))",
          foreground: "hsl(var(--accent-foreground))",
        },
        popover: {
          DEFAULT: "hsl(var(--popover))",
          foreground: "hsl(var(--popover-foreground))",
        },
        card: {
          DEFAULT: "hsl(var(--card))",
          foreground: "hsl(var(--card-foreground))",
        },
      },
      borderRadius: {
        xl: "calc(var(--radius) + 4px)",
        lg: "var(--radius)",
        md: "calc(var(--radius) - 2px)",
        sm: "calc(var(--radius) - 4px)",
      },
      keyframes: {
        grow: {
          '0%': { height: '0' },
          '100%': { height: '100%' },
        },
        fadeIn: {
          '0%': { opacity: '0' },
          '100%': { opacity: '100%' }
        }
      },
      animation: {
        grow: 'grow 1s normal ease-in-out',
        fadeIn: 'fadeIn 1s ease-in-out'
      }
    },
  },
  plugins: [
    plugin(function ({addComponents, theme}) {
      addComponents({
        '.scrollbar-vertical': {
          overflowY: 'auto',
          '&::-webkit-scrollbar': {
            width: '0.5rem',
          },
          '&::-webkit-scrollbar-track': {
            width: '0.5rem',
            paddingLeft: '0.5rem',
          },
          '&::-webkit-scrollbar-thumb': {
            backgroundColor: theme('colors.greyAccent'),
            borderRadius: theme('borderRadius.lg'),
          }
        },
        '.scrollbar-horizontal': {
          overflowX: 'auto',
          paddingBottom: '0.5rem',
          '&::-webkit-scrollbar': {
            height: '0.5rem',
          },
          '&::-webkit-scrollbar-track': {
            height: '0.5rem',
          },
          '&::-webkit-scrollbar-thumb': {
            backgroundColor: theme('colors.greyAccent'),
            borderRadius: theme('borderRadius.lg'),
          }
        }
      });
    })
  ],
} satisfies ModuleOptions['config'];