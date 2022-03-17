export const genInitialsAvatar = ({ name, background }) => {
  background = background || 'e24c4c';
  return `https://ui-avatars.com/api/?name=${name}&background=${background}`;
};
