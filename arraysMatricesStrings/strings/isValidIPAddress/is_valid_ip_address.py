def is_valid_ip_address(IP):
    # try a number of tests of the validity of the IP address. If any fail, raise an exception
    try:
        dotSplit = IP.split('.')
        # test for IPv4 format
        if len(dotSplit) == 4:
            for number in dotSplit:
                # test the length of the number - reject 127... or 1270.0.0.0
                length = len(number)
                if length == 0 or length > 3:
                    raise Exception
                # test for leading zeroes or negatives
                if int(number[0]) == '0' and length > 1:
                    raise Exception
                integer = int(number)
                # test for integers outside the range
                if integer < 0 or integer > 255:
                    raise Exception
            return 'IPv4'
        colonSplit = IP.split(':')
        # test for IPv6 format
        if len(colonSplit) == 8:
            hex_chars = set(['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F'])
            for number in colonSplit:
                length = len(number)
                # test for valid length
                if length < 1 or length > 4:
                    raise Exception
                # test for valid content
                for char in number:
                    if char.upper() not in hex_chars:
                        raise Exception
            return 'IPv6'
        # if neither IPv4 or IPv6 pattern is matched, raise an exception
        raise Exception
    # if an exception is raised, return 'Neither'
    except:
        return 'Neither'