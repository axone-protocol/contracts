import * as React from 'react';
import Box from '@mui/material/Box';
import Button from '@mui/material/Button';
import Modal from '@mui/material/Modal';
import TextField from '@mui/material/TextField';
import InputLabel from '@mui/material/InputLabel';
import MenuItem from '@mui/material/MenuItem';
import FormControl from '@mui/material/FormControl';
import Select, { SelectChangeEvent } from '@mui/material/Select';

const style = {
  position: 'absolute' as 'absolute',
  top: '50%',
  left: '50%',
  transform: 'translate(-50%, -50%)',
  width: 400,
  bgcolor: 'background.paper',
  border: '2px solid #000',
  boxShadow: 24,
  p: 4,
};

const NFTModal = () => {
  const [open, setOpen] = React.useState(false);
  const handleOpen = () => setOpen(true);
  const handleClose = () => setOpen(false);

  return (
    <div>
      <Button onClick={handleOpen}>Open modal</Button>
      <Modal
        open={false}
        onClose={handleClose}
        aria-labelledby="modal-modal-title"
        aria-describedby="modal-modal-description"
      >
        <Box sx={style}>
          <h2>Create NFT</h2>
          <Box sx={{ minWidth: 120 }}>
            <TextField id="filled-basic" label="Price" variant="filled" fullWidth={true}/>
            <br />
            <br />
            <TextField id="filled-basic" label="Description" variant="filled" fullWidth={true}/>
            <br />
            <br />
            <Box>
              <Button variant="outlined" component="label" fullWidth={true}>
                File
                <input hidden accept="image/*" multiple type="file" />
              </Button>
            </Box>
            <br />
            <Box>
              <Button variant="contained" fullWidth={true} style={{backgroundColor: "#0f224a"}}>Mint</Button>
            </Box>
          </Box>
        </Box>
      </Modal>
    </div>
  );
}

export default NFTModal;