import * as React from 'react';
import Box from '@mui/material/Box';
import Button from '@mui/material/Button';
import Modal from '@mui/material/Modal';
import TextField from '@mui/material/TextField';
import InputLabel from '@mui/material/InputLabel';
import MenuItem from '@mui/material/MenuItem';
import FormControl from '@mui/material/FormControl';
import Select, { SelectChangeEvent } from '@mui/material/Select';
import Divider from '@mui/material/Divider';
import TableContainer from '@mui/material/TableContainer';
import Table from '@mui/material/Table';
import TableHead from '@mui/material/TableHead';
import TableRow from '@mui/material/TableRow';
import TableBody from '@mui/material/TableBody';
import TableCell, { tableCellClasses } from '@mui/material/TableCell';
import styled from '@emotion/styled';
import Paper from '@mui/material/Paper';

const style = {
  position: 'absolute' as 'absolute',
  top: '50%',
  left: '50%',
  transform: 'translate(-50%, -50%)',
  width: 1000,
  bgcolor: 'background.paper',
  border: '2px solid #000',
  boxShadow: 24,
  p: 4,
};
const StyledTableCell = styled(TableCell)(({ theme }) => ({
  [`&.${tableCellClasses.head}`]: {
    backgroundColor: "black",
    color: "white",
  },
  [`&.${tableCellClasses.body}`]: {
    fontSize: 14,
  },
}));

const StyledTableRow = styled(TableRow)(({ theme }) => ({
  '&:nth-of-type(odd)': {
    backgroundColor: "white",
  },
  // hide last border
  '&:last-child td, &:last-child th': {
    border: 0,
  },
}));

function createData(
  id: string,
  owner: string,
  is_pinned: boolean,
  size: number,
  compressed_size: number,
) {
  return { id, owner, is_pinned, size, compressed_size };
}

const rows: any = [];

const ObjectariumModal = () => {
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
        <h2>Objectarium Store</h2>
          <Box sx={{ minWidth: 800, display: "flex" }}>
            <FormControl fullWidth style={{ width: "8em", marginRight: "1.5em" }}>
              <InputLabel id="demo-simple-select-label">Object Actions</InputLabel>
              <Select
                style={{ width: "7.5em" }}
                labelId="demo-simple-select-label"
                id="demo-simple-select"
                value={"age"}
                label="object-actions"
              // onChange={handleChange}
              >
                <MenuItem value={10}>STORE</MenuItem>
                <MenuItem value={20}>FORGET</MenuItem>
                <MenuItem value={30}>PIN</MenuItem>
                <MenuItem value={40}>UNPIN</MenuItem>
              </Select>
            </FormControl>
            <TextField id="filled-basic" label="id" variant="filled" fullWidth={true} />
          </Box>
          <br />

          <Box>
            <Button variant="contained" style={{ height: 56, backgroundColor: "#0f224a" }} fullWidth={true}>Execute</Button>
          </Box>
          <br />
          <Divider>Results</Divider>
          <br />
          {/* Table results */}
          <TableContainer component={Paper}>
      <Table sx={{ minWidth: 700 }} aria-label="customized table">
        <TableHead>
          <TableRow>
            <StyledTableCell>id</StyledTableCell>
            <StyledTableCell align="right">owner</StyledTableCell>
            <StyledTableCell align="right">is_pinned</StyledTableCell>
            <StyledTableCell align="right">size</StyledTableCell>
            <StyledTableCell align="right">compressed_size</StyledTableCell>
          </TableRow>
        </TableHead>
        <TableBody>
          {rows.map((row) => (
            <StyledTableRow key={row.id}>
              <StyledTableCell component="th" scope="row">
                {row.id}
              </StyledTableCell>
              <StyledTableCell align="right">{row.owner}</StyledTableCell>
              <StyledTableCell align="right">{row.is_pinned}</StyledTableCell>
              <StyledTableCell align="right">{row.size}</StyledTableCell>
              <StyledTableCell align="right">{row.compressed_size}</StyledTableCell>
            </StyledTableRow>
          ))}
        </TableBody>
      </Table>
    </TableContainer>

        </Box>
      </Modal>
    </div>
  );
}

export default ObjectariumModal;