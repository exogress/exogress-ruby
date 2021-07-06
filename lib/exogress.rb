# frozen_string_literal: true

require_relative "exogress/version"
require 'rutie'

class ExogressError < StandardError
end

class EntityError < StandardError
end

module Exogress
  Rutie.new(:rutie_exogress).init 'Init_rutie_exogress', __dir__
end
