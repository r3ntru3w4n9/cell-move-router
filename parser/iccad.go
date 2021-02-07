package parser

import (
	"strings"
)

// IccadParser parses data from ICCAD2020 challenge into a 2d structure
type IccadParser struct {
}

// NewIccadParser creates a new ICCAD2DParser
func NewIccadParser() *IccadParser {
	return &IccadParser{}
}

// Parse parses the data from the file
func (parser *IccadParser) Parse(content []string) error {
	iter := NewIterator(content)

	parser.skipNonRoutingParts(iter)
	parser.parseRoutingPart(iter)

	if iter.HasNext() {
		return ErrUnfinished
	}

	return nil
}

func (parser *IccadParser) skipNonRoutingParts(iter *Iterator) {
	// MaxCellMove <maxMoveCount>
	AssertEqual(iter.Next(), "MaxCellMove")
	Atoi(iter.Next())

	// GGridBoundaryIdx <rowBeginIdx> <colBeginIdx> <rowEndIdx> <colEndIdx>
	AssertEqual(iter.Next(), "GGridBoundaryIdx")
	// start and end are both equal to 1
	AssertEqual(Atoi(iter.Next()), 1)
	AssertEqual(Atoi(iter.Next()), 1)

	Atoi(iter.Next())
	Atoi(iter.Next())

	// NumLayer <LayerCount>
	AssertEqual(iter.Next(), "NumLayer")
	numLayers := Atoi(iter.Next())

	// Lay <layerName> <Idx> <RoutingDirection> <defaultSupplyOfOneGGrid>
	for i := 0; i < numLayers; i++ {
		AssertEqual(iter.Next(), "Lay")
		iter.Next()
		Atoi(iter.Next())
		iter.Next()
		Atoi(iter.Next())
	}

	// NumNonDefaultSupplyGGrid <nonDefaultSupplyGGridCount>
	AssertEqual(iter.Next(), "NumNonDefaultSupplyGGrid")
	numNonDefault := Atoi(iter.Next())

	// <rowIdx> <colIdx> <LayIdx> <incrOrDecrValue>
	for i := 0; i < numNonDefault; i++ {
		Atoi(iter.Next())
		Atoi(iter.Next())
		Atoi(iter.Next())
		Atoi(iter.Next())
	}

	// NumMasterCell <masterCellCount>
	AssertEqual(iter.Next(), "NumMasterCell")
	numMasterCell := Atoi(iter.Next())

	// MasterCell <masterCellName> <pinCount> <blockageCount>
	for i := 0; i < numMasterCell; i++ {
		AssertEqual(iter.Next(), "MasterCell")
		iter.Next()
		pinCount := Atoi(iter.Next())
		blkgCount := Atoi(iter.Next())

		// Pin <pinName> <pinLayer>
		for j := 0; j < pinCount; j++ {
			AssertEqual(iter.Next(), "Pin")
			iter.Next()
			iter.Next()
		}

		// Blkg <blockageName> <blockageLayer> <demand>
		for j := 0; j < blkgCount; j++ {
			AssertEqual(iter.Next(), "Blkg")
			iter.Next()
			iter.Next()
			Atoi(iter.Next())
		}
	}

	// NumNeighborCellExtraDemand <count>
	AssertEqual(iter.Next(), "NumNeighborCellExtraDemand")
	extraCount := Atoi(iter.Next())

	// sameGGrid <masterCellName1> <masterCellName2> <layerName> <demand>
	// adjHGGrid <masterCellName1> <masterCellName2> <layerName> <demand>
	for i := 0; i < extraCount; i++ {
		switch ggrid := iter.Next(); ggrid {
		case "sameGGrid", "adjHGGrid":
		default:
			Unreachable()
		}

		iter.Next()
		iter.Next()
		iter.Next()
		Atoi(iter.Next())
	}

	// NumCellInst <cellInstCount>
	AssertEqual(iter.Next(), "NumCellInst")
	cellCount := Atoi(iter.Next())

	// CellInst <instName> <masterCellName> <gGridRowIdx> <gGridColIdx> <movableCstr>
	for i := 0; i < cellCount; i++ {
		AssertEqual(iter.Next(), "CellInst")
		iter.Next()
		iter.Next()
		Atoi(iter.Next())
		Atoi(iter.Next())
		iter.Next()
	}
}

func (parser *IccadParser) parseRoutingPart(iter *Iterator) {
	// NumNets <netCount>
	AssertEqual(iter.Next(), "NumNets")
	netCount := Atoi(iter.Next())

	// allNets := make(map[int][]int, 0)

	// Net <netName> <numPins> <minRoutingLayConstraint>
	for i := 0; i < netCount; i++ {
		AssertEqual(iter.Next(), "Net")
		netName := iter.Next()
		AssertEqual(netName[:1], "N")
		netID := Atoi(netName[1:]) - 1
		AssertEqual(i, netID)

		numPins := Atoi(iter.Next())
		iter.Next()

		// Pin <instName>/<masterPinName>
		for j := 0; j < numPins; j++ {
			AssertEqual(iter.Next(), "Pin")
			bothNames := strings.Split(iter.Next(), "/")
			AssertEqual(len(bothNames), 2)
			pinName := bothNames[0]
			pinID := Atoi(pinName) - 1
			AssertEqual(pinID, j)
		}
	}

	// NumRoutes <routeSegmentCount>
	AssertEqual(iter.Next(), "NumRoutes")
	routeSegCount := Atoi(iter.Next())

	// <sRowIdx> <sColIdx> <sLayIdx> <eRowIdx> <eColIdx> <eLayIdx> <netName>
	for i := 0; i < routeSegCount; i++ {
		sRow := Atoi(iter.Next())
		sCol := Atoi(iter.Next())
		sLay := Atoi(iter.Next())
		eRow := Atoi(iter.Next())
		eCol := Atoi(iter.Next())
		eLay := Atoi(iter.Next())
		AssertTrue(sRow == eRow || sCol == eCol || sLay == eLay)
		iter.Next()
	}
}

// Result displays the result of that got parsed
func (parser IccadParser) Result() {
	Todo("represent the whole grid")
}
